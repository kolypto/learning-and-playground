import 'package:flutter/material.dart';
import 'package:http/http.dart' as http;

void main() {
  runApp(const MainApp());
}

class MainApp extends StatelessWidget {
  const MainApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      theme: ThemeData(
        useMaterial3: true,
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.blue),
      ),
      home: Scaffold(
        body: Center(
            child: Center(
          child: ElevatedButton(
              onPressed: () {
                print("pressed");
                sendRequest();
              },
              child: Text("Голубая Кнопка", textScaleFactor: 3)),
        )),
      ),
    );
  }
}

sendRequest() async {
  sendTelegramBotMessage('691814383', ':)'); // Me
}

sendTelegramBotMessage(String chatId, String text) async {
  Map data = {
    'chat_id': chatId,
    'text': text,
  };
  const botKey = 'bot1842331905:AA....';
  var url = Uri.parse('https://api.telegram.org/$botKey/sendMessage');
  http.post(url, body: data).then((response) {
    print("Response status: ${response.statusCode}");
    print("Response body: ${response.body}");
  });
}
