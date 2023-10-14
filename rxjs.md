Tired of comparing them:

* Observable: unicast: each subscribed Observer owns an independent execution of the Observable.
    That is, every subscriber starts their own "thread".

* Subject: broadcast new values. No values = wait for them. Subjects are multicast: subscribe() does not invoke a new execution.
    It's an observer: Unlike observables, `Subject.next()` is available, and you can do `Observable.subscribe(subject)` to send values there.

* BehaviorSubject(default): starts with a default value: no wait.
* ReplaySubject: broadcast new values, but also replay the most recent N values: no wait for new values.

