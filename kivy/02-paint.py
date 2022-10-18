from random import random
from kivy.app import App 
from kivy.uix.widget import Widget
from kivy.uix.button import Button
from kivy.input.motionevent import MotionEvent
from kivy.graphics import Color, Ellipse, Line

class PaintWidget(Widget):
    """ The drawing area """ 

    # On touch: add circle, add line
    def on_touch_down(self, touch: MotionEvent):
        with self.canvas:
            # Choose color, size
            Color(random(), 1, 1, mode='hsv')
            d = 30

            # Add a circle
            Ellipse(pos=(touch.x - d/2, touch.y - d/2), size=(d, d))

            # Add a line, with 1 points yet.
            # Stash the line into a `ud` use dictionary
            touch.ud['line'] = Line(points=(touch.x, touch.y))

    def on_touch_move(self, touch: MotionEvent):
        # NOTE: this is the *same* touch event!
        # Get the line, add another point. It'll be redrawn.
        touch.ud['line'].points += (touch.x, touch.y)

class PaintApp(App):
    kv_file = '02-paint.kv'

    def build(self):
        # return PaintWidget()
        parent = Widget()

        self.painter = PaintWidget()
        parent.add_widget(self.painter)

        clear_btn = Button(text='Clear')
        clear_btn.bind(on_release=self.clear_canvas)
        parent.add_widget(clear_btn)

        return parent

    def clear_canvas(self, obj):
        self.painter.canvas.clear()

if __name__ == '__main__':
    PaintApp().run()
