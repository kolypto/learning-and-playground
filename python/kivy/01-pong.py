from __future__ import annotations

import datetime
from random import randint
import kivy
import kivy.event
import kivy.clock
import kivy.app
import kivy.vector
import kivy.uix.widget

class PongApp(kivy.app.App):
    """ The Application """

    # The kv file to load the UI from
    kv_file = '01-pong.kv'

    # Called once when the application is initialized
    def build(self):
        game = PongGame()

        # Init the ball's speed
        game.serve_ball()

        # Schedule a function to be called regularly
        kivy.clock.Clock.schedule_interval(game.update, 1/60)

        return game


class PongGame(kivy.uix.widget.Widget):
    # Ball object. Hooked up in the kv file.
    ball: PongBall = kivy.uix.widget.ObjectProperty(None)

    # Players
    player1: PongPaddle = kivy.uix.widget.ObjectProperty(None)
    player2: PongPaddle = kivy.uix.widget.ObjectProperty(None)

    def serve_ball(self):
        """ Init ball: center, speed, random direction """
        self.ball.center = self.center
        self.ball.velocity = kivy.vector.Vector(4, 0).rotate(randint(0, 360))

    def update(self, dt: datetime.timedelta):
        """ On timer: move the ball """
        self.ball.move()
        
        # Bounce off paddles
        self.player1.bounce_ball(self.ball)
        self.player2.bounce_ball(self.ball)

        # Bounce off top and bottom
        if self.ball.y < 0 or self.ball.top > self.height:
            self.ball.velocity_y *= -1

        # # Bounce off left and right
        # if self.ball.x < 0 or self.ball.right > self.width:
        #     self.ball.velocity_x *= -1

        # Went off to a side to score a point?
        if self.ball.x < self.x:
            self.player2.score += 1
            self.serve_ball()
        
        if self.ball.right > self.width:
            self.player1.score += 1
            self.serve_ball()

    def on_touch_move(self, touch: kivy.event.MouseMotionEvent):
        if touch.x < self.width/3:
            self.player1.center_y = touch.y
        if touch.x > (self.width - self.width/3):
            self.player2.center_y = touch.y


class PongBall(kivy.uix.widget.Widget):
    velocity_x = kivy.uix.widget.NumericProperty(0)
    velocity_y = kivy.uix.widget.NumericProperty(0)
    velocity = kivy.uix.widget.ReferenceListProperty(velocity_x, velocity_y)

    # Will be called in equal intervals to animate the ball
    def move(self):
        """ On timer: change the ball's location """
        self.pos = kivy.vector.Vector(*self.velocity) + self.pos


class PongPaddle(kivy.uix.widget.Widget):
    score = kivy.uix.widget.NumericProperty(0)

    def bounce_ball(self, ball: PongBall):
        if self.collide_widget(ball):
            speedup = 1.1

            # The ball bounces differently depending on where it hits the racket
            offset = 0.02 * kivy.vector.Vector(0, ball.center_y - self.center_y)
            ball.velocity = speedup * (offset - ball.velocity)


if __name__ == '__main__':
    PongApp().run()
