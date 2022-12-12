from kivy.app import App 
from kivy.uix.widget import Widget, NumericProperty, StringProperty


class ManualApp(App):
    kv_file = '03-manual.kv'

    def build(self):
        return LoginScreen()


class LoginScreen(Widget):
    # Widget is an EventDispatcher.
    # Kivy properties are Observables. NOTE: only usable at class level!
    # f_login = StringProperty()
    # f_passwd = StringProperty()
    pass


if __name__ == '__main__':
    ManualApp().run()
