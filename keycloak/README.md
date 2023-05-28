# KeyCloak

KeyCloak: OAuth2 + OpenID Connect with a database of users.

Alternatives:

* <https://github.com/ory/hydra>
* <https://github.com/zitadel/zitadel>
* <https://github.com/casdoor/casdoor>

# Keycloak

Start it with docker-compose.

## Set Up

Create realm:

* name: demo

Create client:

* Client type: OpenID connect
* Client id: backend
* Name: backend
* Client authentication: yes
* Authorization: yes

Also configure:

* Redirect Uri: set "http://example.com/*" or something like this. If you don't, `redirect_uri` won't work.
* Web origins: URL patterns for CORS: set "https://example.com"

Test configuration with the web app: <https://www.keycloak.org/app/>

Credentials tab: Get the "client secret" (in testing, you can define your own simple key)

## Export the realm

Then go to Authorization/Policies and remove the default "JS" policy (to make the realm exportable).

Now go to "Realm settings" and do Action->"Partial Export", with all options enabled.
Then take the file and bind-mount it to `/opt/keycloak/data/import/realm-export.json`.

Note that the secret is not exported:

```json
{
  "clients": [
    {
      "clientId": "backend",
      "secret": "**********",
    }
  ]
}
```

Never define this secret in production! It will be generated anew.

## CLI usage

See OpenID configuration and endpoints:
<http://localhost:8282/realms/demo/.well-known/openid-configuration>

```js
{
"issuer": "http://keycloak.localhost:8282/realms/demo",
"authorization_endpoint": "http://keycloak.localhost:8282/realms/demo/protocol/openid-connect/auth",
"token_endpoint":         "http://keycloak.localhost:8282/realms/demo/protocol/openid-connect/token",
"introspection_endpoint": "http://keycloak.localhost:8282/realms/demo/protocol/openid-connect/token/introspect",
"userinfo_endpoint":      "http://keycloak.localhost:8282/realms/demo/protocol/openid-connect/userinfo",
"end_session_endpoint":   "http://keycloak.localhost:8282/realms/demo/protocol/openid-connect/logout",
  //...
}
```

Get an *access token* outside of the context of a user (i.e. not signed in, acting as a system user):

```console
$ http --form POST 'http://localhost:8282/realms/demo/protocol/openid-connect/token' client_id=backend client_secret=UcYQS1Zdi8tJ608XTVGnJbw479QS4XcB grant_type=client_credentials
{
  "access_token":"eyJhbGciOiJSUzI1NiIsInR5c...",
  "expires_in":300,
  "refresh_expires_in":0,
  "token_type":"Bearer",
  "not-before-policy":0,
  "scope":"profile email"
}
```

Get an *access token* for a user account using "password" flow:

```console
$ http --form POST 'http://localhost:8282/realms/demo/protocol/openid-connect/token' client_id=backend client_secret=$client_secret scope="openid email profile" grant_type=password username=user@example.com password=admin

{
    "access_token": "eyJhbGc...",
    "expires_in": 300,
    "not-before-policy": 0,
    "refresh_expires_in": 1800,
    "refresh_token": "eyJhbGciOi...",
    "scope": "email profile",
    "session_state": "59cec4fd-6c6f-4aa2-84de-4e09ed5bf5a7",
    "token_type": "Bearer"
}
```

And refresh it:

```console
$ http --form POST 'http://localhost:8282/realms/demo/protocol/openid-connect/token' client_id=backend client_secret=$client_secret grant_type=refresh_token refresh_token=$refresh_token
```

Get user info: (make sure that the "openid" scope is granted)

```console
$ http --form POST 'http://localhost:8282/realms/demo/protocol/openid-connect/userinfo' client_id=backend client_secret=$client_secret Authorization:"Bearer $access_token"

{
    "email": "user@example.com",
    "email_verified": true,
    "family_name": "Smith",
    "given_name": "John",
    "name": "John Smith",
    "preferred_username": "user@example.com",
    "sub": "d389d885-d92d-401c-9577-6b4d23d7bf00"
}
```

Same information can be obtained using token introspection:

```console
$ http --form POST 'http://localhost:8282/realms/demo/protocol/openid-connect/token/introspect' client_id=backend client_secret=$client_secret token=$access_token

// ID Token
{
    "typ": "Bearer",
    // Generated for Audience: client_id of the Relying Party (the one who gets the token)
    "aud": "account",
    // Authorized party: the party to which the ID Token was issued
    "azp": "backend",
    "client_id": "backend",
    // Issuing authority
    "iss": "http://localhost:8282/realms/demo",

    // User id
    "sub": "d389d885-d92d-401c-9577-6b4d23d7bf00",
    // Session id
    "sid": "1d59a730-5f7a-4f14-9a92-10403a72f135",
    // Issue time
    "iat": 1684265858,
    // Expiration time
    "exp": 1684266158,

    // Authentication Context Class Reference:
    "acr": "1",
    "jti": "e5898b6a-7c23-4ee7-afdf-89e2f2636916",
    "active": true,
    "allowed-origins": [
        "/*"
    ],
    "realm_access": {
        "roles": [
            "offline_access",
            "superadmin",
            "uma_authorization",
            "default-roles-demo"
        ]
    },
    "resource_access": {
        "account": {
            "roles": [
                "manage-account",
                "manage-account-links",
                "view-profile"
            ]
        }
    },
    "scope": "openid email profile",
    "session_state": "1d59a730-5f7a-4f14-9a92-10403a72f135",

    "username": "user@example.com"
        "email": "user@example.com",
    "email_verified": true,
    "family_name": "Smith",
    "given_name": "John",
    "name": "John Smith",
    "preferred_username": "user@example.com",

}

```

Or sign in using web interface:

> http://localhost:8282/realms/demo/protocol/openid-connect/auth?response_type=code&client_id=backend

> http://localhost:8282/realms/demo/protocol/openid-connect/auth?response_type=code&client_id=backend&scope=openid email&redirect_uri=http://example.com&state=1234

* redirect_uri: this is where the user is sent upon successful authentication
* state: random number checked by the client (to prevent CSRF attacks)
* When redirected, "?code=" is added to the URL

Use this code to get the token:

```console
$ http --form POST 'http://localhost:8282/realms/demo/protocol/openid-connect/token' client_id=backend client_secret=$client_secret grant_type=authorization_code code=c3d06...
```

## Add Claims (attributes) to the token

To expose an attribute:

* Add the attribute to a user / user's group
* Configure a client scope's "mapper" to expose it ("claim")
* The attribute ("claim") is now added to the JWT
* If you created a custom client scope, be sure to add it to client "scopes"

Note that a User inherits all attributes from a Group.
They do not (yet) inherit attributes from a role, though.












# OAuth2 + OIDC

To request an "ID Token", the client ("Relying Party", RP) goes to the IdP ("Identity Provider") and provides credentials. This role is usually performed by the web browser: a popup.

OAuth2 has flows:

* *Authorization code flow*: a code is generated, and retrieved by the server. The auth token is not revealed to the browser.
* *Implicit Flow*: for JS apps that do not have a back-end
* *Hybrid Flow*: rarely used, allows the application front-end and back-end to receive tokens separately from one another. Essentially a combination of the code and implicit flows.

A minimal example of how to obtain an ID token for a user from an OP ("OpenID provider") using the *authorization code flow*: the most commonly used flow by traditional web apps.

## Flow: Authorization Code

First step: send the browser to the OAuth2 authorization endpoint. Use 302 Found redirection:

```
http://keycloak.localhost:8282/realms/demo/protocol/openid-connect/auth
 ?response_type=code
 &scope=openid
 &client_id=backend
 &redirect_uri=http://example.com/cb
 &state=123456
```

If you're getting this error, make sure you whitelist the "Valid redirect URIs" client parameter:

> Invalid parameter: redirect_uri

The user may have to sign in, answer some questions (agree to sign into the RP), and then the user will be redirected to:

```
https://example.com/cb
 ?session_state=e3cbfdca-c6d4-460b-bde1-fc17dd645fc5
 &code=25e1ccc3-2be8-4897-b80c-425f1be6afc9.e3cbfdca-c6d4-460b-bde1-fc17dd645fc5.4bd3ebff-7df0-4cac-b1b6-00d6321bf98f
 &state=123456
```


In case of an error, the user is sent to:

```
http://example.com/
  #error=invalid_request
  &error_description=Missing parameter: nonce
  &state=123456
```


The `state` parameter must be validated: to make sure it's the same one (CSRF protection).

Now the server will exchange the `code` for an ID token. You have to provide the same `redirect_uri` as well:

```console
$ set code "25e1ccc3-2be8-4897-b80c-425f1be6afc9.e3cbfdca-c6d4-460b-bde1-fc17dd645fc5.4bd3ebff-7df0-4cac-b1b6-00d6321bf98f"
$ http --form POST 'http://localhost:8282/realms/demo/protocol/openid-connect/token' \
  client_id=backend client_secret=$client_secret \
  grant_type=authorization_code redirect_uri=http://example.com/ code=$code

{
    "id_token": "...",
    "access_token": "...",
    "expires_in": 300,
    "not-before-policy": 0,
    "refresh_expires_in": 1800,
    "refresh_token": "...",
    "scope": "openid email profile",
    "session_state": "e3cbfdca-c6d4-460b-bde1-fc17dd645fc5",
    "token_type": "Bearer"
}
```

It has two tokens:

* The *identity token* contains information about the user such as username, email, and other profile information.
* The *access token* is digitally signed by the realm and contains access information (like user role mappings) that the application can use to determine what resources the user is allowed to access on the application.

The ID token has:

```js
{
  "exp": 1684414156,
  "iat": 1684413856,
  "auth_time": 1684412858,
  "jti": "a8fbe8e9-4828-4f0a-a66c-d80205ac91b4",
  "iss": "http://keycloak.localhost:8282/realms/demo",
  "aud": "backend",
  "sub": "d389d885-d92d-401c-9577-6b4d23d7bf00",
  "typ": "ID",
  "azp": "backend",
  "session_state": "e3cbfdca-c6d4-460b-bde1-fc17dd645fc5",
  "at_hash": "zCXB36w05G_Cd0_vLvt5Lg",
  "acr": "0",

  "sid": "e3cbfdca-c6d4-460b-bde1-fc17dd645fc5",
  "email_verified": true,
  "name": "John Smith",
  "preferred_username": "user@example.com",
  "given_name": "John",
  "family_name": "Smith",
  "email": "user@example.com"
}
```

The access token has this in addition:

```js
{
  // as above, plus:
  "aud": "account",
  "typ": "Bearer",
  "allowed-origins": [
    "/*"
  ],
  "realm_access": {
    "roles": [
      "offline_access",
      "superadmin",
      "uma_authorization",
      "default-roles-demo"
    ]
  },
  "resource_access": {
    "account": {
      "roles": [
        "manage-account",
        "manage-account-links",
        "view-profile"
      ]
    }
  },
}
```

The fields are called "claims". OpenID specified these standard claims:

* scope=email: `email`, `email_verified`
* scope=phone: `phone`', `phone_verified`
* scope=profile: `name` (full), `family_name` + `given_name`, `middle_name`, `nickname`
* scope=profile: `preferred_username`
* scope=profile: `profile`, `picture`, `website`, `gender`, `birthdate`, `zoneinfo`, `locale`, `updated_at`
* scope=address: `address`

You can request a claim by specifying a "scope=email phone" or individually, by setting "claims=name picture".
Note that even your application has requested a scope, the user may choose to deny release of some claims.

The `/userinfo` endpoint returns previously consented user profile information to the client:

```console
$ set access_token "..."
$ http GET 'http://localhost:8282/realms/demo/protocol/openid-connect/userinfo' Authorization:"Bearer $access_token"
{
    "email": "kolypto@gmail.com",
    "email_verified": true,
    "family_name": "Vartanyan",
    "given_name": "Mark",
    "name": "Mark Vartanyan",
    "phone_number": "123",
    "preferred_username": "kolypto@gmail.com",
    "sub": "d389d885-d92d-401c-9577-6b4d23d7bf00"
}
```


## Flow: Implicit Flow (Token)

NOTE: Deprecated! Because it exposes the access token to the browser!

The implicit flow is used for SPA applications with no back-end. The app will get he `access_token` right away.

To enable the "implicit flow", create a Client with Implicit Flow enabled.

Then:

```
http://keycloak.localhost:8282/realms/demo/protocol/openid-connect/auth
 ?response_type=token id_token
 &scope=openid email
 &client_id=frontend
 &redirect_uri=http://example.com/
 &state=123456
 &nonce=789
```

Here, "nonce" is a cryptographically secure random string (to mitigate replay attacks)

When the user signs in, they are sent to:

```
http://example.com/
 #state=123456
 &session_state=cfdc85cd-a374-4186-a2e5-d2c6800d085a
 &id_token=eyJhbGciO....
 &access_token=eyJhbG...
 &token_type=Bearer
 &expires_in=900
```

In case of an error, the user is sent to:

```
http://example.com/
  #error=invalid_request
  &error_description=Missing parameter: nonce
  &state=123456
```

## Flow: Authorization Code + PKCE

Authorization Code Flow with Proof Key for Code Exchange (PKCE).

TODO: read & describe


## Flow: Refresh Token

Send the `refresh_token` and get a new `access_token`:

```console
$ http --form POST 'http://localhost:8282/realms/demo/protocol/openid-connect/token' client_id=backend client_secret=$client_secret grant_type=refresh_token refresh_token=$refresh_token

{
    "access_token": "eyJhbGciOiJ...",
    "expires_in": 300,
    "not-before-policy": 1684419856,
    "refresh_expires_in": 1800,
    "refresh_token": "eyJhbGc...",
    "scope": "email user_groups profile",
    "session_state": "b990cd95-52b7-4f94-b442-f178f20a1957",
    "token_type": "Bearer"
}
```

If you have requested the `offline_access` scope, then `refresh_token` never expires: can be used to access the account when the user is not available, e.g. for background jobs:

```console
$ http --form POST 'http://localhost:8282/realms/demo/protocol/openid-connect/token' client_id=backend client_secret=$client_secret grant_type=password username=kolypto@gmail.com password=admin scope="openid offline_access"

{
    "access_token": "...",
    "expires_in": 1800,
    "id_token": "...",
    "not-before-policy": 1684419856,
    "refresh_expires_in": 0,   // <-- !!!
    "refresh_token": "eyJhbGci...",
    "scope": "openid attributes email user_groups profile offline_access",
    "session_state": "737104ef-4970-41ea-a406-a5af05cd7e13",
    "token_type": "Bearer"
}
```

Here's how to check it:

```console
$ set offline_token (http --form POST 'http://localhost:8282/realms/demo/protocol/openid-connect/token' client_id=backend client_secret=$client_secret grant_type=password username=kolypto@gmail.com password=admin scope="openid offline_access" | jq .access_token -r)
$ http GET 'http://localhost:8282/realms/demo/protocol/openid-connect/userinfo' Authorization:"Bearer $offline_token"
```

## Flow: Password Flow

Password flow is simple: provide a login+password, get an access token in exchange:

```console
$ http --form POST 'http://localhost:8282/realms/demo/protocol/openid-connect/token' client_id=backend client_secret=$client_secret grant_type=password username=kolypto@gmail.com password=admin scope="openid email user_groups"

{
    "access_token": "eyJhbGciOiJSUzI...",
    "expires_in": 300,
    "id_token": "eyJhbGciOi...",
    "not-before-policy": 1684419856,
    "refresh_expires_in": 1800,
    "refresh_token": "eyJhbGciOi...",
    "scope": "openid email user_groups profile",
    "session_state": "e30445c6-64d8-4ada-8efb-0255368ce945",
    "token_type": "Bearer"
}

```

## Flow: Device Authorization

With input-constrained devices that connect to the internet, rather than authenticate the user directly, the device asks the user to go to a link on their computer or smartphone and authorize the device. This avoids a poor user experience for devices that do not have an easy way to enter text.

Create a "client" with "OAuth2.0 Device Authorization" grant.

The device connects to the OAuth2 server and gets a unique code:

```console
$ http --form POST 'http://localhost:8282/realms/demo/protocol/openid-connect/auth/device' client_id=backend client_secret=$client_secret scope="openid email"
{
    "device_code": "29jFz3tQNxjAB23QhlY4eCfwCSn4g8Mq7SFO1BiPgcQ",
    "expires_in": 600,
    "interval": 5,
    "user_code": "EVFG-JGHP",
    "verification_uri": "http://keycloak.localhost:8282/realms/demo/device",
    "verification_uri_complete": "http://keycloak.localhost:8282/realms/demo/device?user_code=EVFG-JGHP"
}
```

Now the device is supposed to tell the user:

* `verification_uri` and `user_code`; or
* `verification_uri_complete`

The short URL + `device_code` can be displayed on the screen, or `verification_uri_complete` can be displayed as QR code.

While the user is signing in, the device would poll the endpoint every `interval` for `expires_in` seconds:

```console
$ http --form POST 'http://localhost:8282/realms/demo/protocol/openid-connect/token' client_id=backend client_secret=$client_secret grant_type=urn:ietf:params:oauth:grant-type:device_code device_code=29jFz3tQNxjAB23QhlY4eCfwCSn4g8Mq7SFO1BiPgcQ
```

It will keep giving you:

```javascript
// HTTP/1.1 400 Bad Request
{
    "error": "authorization_pending",
    "error_description": "The authorization request is still pending"
}
```

or if you're requesting too often:

```javascript
// HTTP/1.1 400 Bad Request

{
  "error": "slow_down"
}
```

but then the response changes to:

```javascript
// HTTP/1.1 200 OK
{
    "access_token": "eyJh...",
    "expires_in": 300,
    "id_token": "eyJhbGc...",
    "not-before-policy": 1684419856,
    "refresh_expires_in": 1800,
    "refresh_token": "eyJhb...",
    "scope": "openid email user_groups profile",
    "session_state": "cfdc85cd-a374-4186-a2e5-d2c6800d085a",
    "token_type": "Bearer"
}
```

Now the device can get your user account information.

Or, if the user has denied the request:

```javascript
// HTTP/1.1 400 Bad Request
{
  "error": "access_denied"
}
```

Or, if the device token has expired:

```js
// HTTP/1.1 400 Bad Request
{
  "error": "expired_token"
}
```


## Flow: CIBA Authorization

CIBA = Client-Initiated Backchannel Authentication.
That is, the user is authenticated via some external authentication device instead of the user's browser.

It's a decoupled flow where authentication can be initiated on onw device and carried out at another.

CIBA opens new possibilities. For example, a call center agent may ask a caller to authenticate via a decoupled login.

TODO: read more

## Flow: Token Exchange

Exhange a token that you have and get a more limited, restricted token.

TODO: see [Token Exchange](https://www.keycloak.org/docs/latest/securing_apps/index.html#_token-exchange)














# Keycloak Authorization

Granting access to resources.

Create Client: enable "authorization".

On the "Authorization" tab:

* Choose "Enforcing" (deny when no policy is associated with a resource) or "Permissive" (allow when no policy is associated)
* Choose "Unanimous" (all permissions must allow) or "Affirmative" (at least one permission must allow)

By default, you get a "Default Resource": represents all resources of your application.
It has a policy that always grants access, and a permission.

* The Resource defines a `Type`: "urn:backend:resources:default". It can be used to create *typed resource permissions*: that apply to multiple resources by type.
* The Resource defines an URI: "/*". The wildcard URI represents all paths in the application.

Create resources:

* Name: human-readable name. Examples: "Admin", "Photo album", ...
* Type: groups resources. Custom string. Examples: 'admin', 'bank:account', 'device:phone'.
* URI: resource locations/addresses. Examples: `/admin/*`, `/album/{id}`
* Authorization scopes: scopes to associate with the resource. Examples: "withdraw", "admin"
* Attributes. Examples: "account.withdraw.limit=100"

Create a policy:

* Grant access to specific users, roles, groups, client scopes
* Name: "Only Admins"

Create a resource permission: a set of resources to protected by an authorization policy.

* Name it: "Admin Resource Permission", "Album Resource Permission"
* Add policies that allow it: "Only admins" + "My user backdoor" :)

Then go to the "evaluate", and choose a User. Your access token would have something like this:

```js
{
  "authorization": {
    "permissions": [
      {
        // Granted access to this resource because of some policy
        "rsid": "e2116885-ceb9-4fc8-82f5-c0825e027ec1",
        "rsname": "Admin"
      }
    ]
  },
}
```

Or Create a scope-based permission: a set of scopes to protect by a policy.
First:

* Create a scope, e.g. "withdraw". Note: it has NOTHING to do with OAuth2 scopes!
  The "scope" is basically the set of actions that you can do upon a resource.
* Create a resource: e.g. "Bank Account", and add this scope to it
* Now create a permission: "Withdraw from Bank Account" with Resource="Bank Account" and scope="Withdraw".

Your access token would now look like this:

```js
{
  "authorization": {
    "permissions": [
      // Permission to a resource
      {
        "rsid": "e2116885-ceb9-4fc8-82f5-c0825e027ec1",
        "rsname": "Admin"
      },
      // Permission to a resource with a specific scope
      {
        "scopes": [
          "withdraw-money"
        ],
        "rsid": "ea25ad23-9355-438e-a451-6eb130396455",
        "rsname": "Bank Account"
      }
    ]
  },
}
```

Permissions are not automatically added to the access token. You've got to request them:

```console
$ set access_token (http --form POST 'http://localhost:8282/realms/demo/protocol/openid-connect/token' client_id=backend client_secret=$client_secret grant_type=password username=kolypto@gmail.com password=admin | jq .access_token -r)

$ http --form POST 'http://localhost:8282/realms/demo/protocol/openid-connect/token' audience=backend Authorization:"Bearer $access_token" grant_type='urn:ietf:params:oauth:grant-type:uma-ticket' permission="bank-account#withdraw-money"

{
    "access_token": "eyJhbGc...",
    "expires_in": 300,
    "not-before-policy": 1684419856,
    "refresh_expires_in": 1800,
    "refresh_token": "eyJhbGci...",
    "token_type": "Bearer",
    "upgraded": false
}
```

The access token would contain the permissions to scopes/resources.

```js
// $ http --form POST 'http://localhost:8282/realms/demo/protocol/openid-connect/token' audience=backend Authorization:"Bearer $access_token" grant_type='urn:ietf:params:oauth:grant-type:uma-ticket' permission="bank-account#withdraw-money" | jq .access_token -r | jwt -show -

{
    "authorization": {
        "permissions": [
            {
                "rsid": "ea25ad23-9355-438e-a451-6eb130396455",
                "rsname": "bank-account",
                "scopes": [
                    "withdraw-money"
                ]
            }
        ]
    },
    //...
}

//... permission="Admin"
{
    "authorization": {
        "permissions": [
            {
                "rsid": "e2116885-ceb9-4fc8-82f5-c0825e027ec1",
                "rsname": "Admin"
            }
        ]
    },

}

//... permission="Album"
// HTTP/1.1 403 Forbidden
{
    "error": "access_denied",
    "error_description": "not_authorized"
}
```

Resources and scopes can be managed by a remote API: see [Managing resources using a UMA-Compliant endpoint](https://www.keycloak.org/docs/latest/authorization_services/index.html#_service_protection_resources_api). That is, you can create a resource with its scopes.


