import 'package:flutter/material.dart';
import 'dart:ffi';
import 'dart:io' show Platform;
import 'bridge_generated.dart';

const buildName = "librust_app.so";
final dyLib = DynamicLibrary.open(buildName);

late final api = RustAppImpl(dyLib);

void main() async {
  runApp(MyApp());
}

class MyApp extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(
          title: Text('Flutter Rust Bridge Emulator'),
        ),
        body: RegisterWidget(),
      ),
    );
  }
}

class RegisterWidget extends StatefulWidget {
  @override
  _RegisterWidgetState createState() => _RegisterWidgetState();
}

class _RegisterWidgetState extends State<RegisterWidget> {
  int _registerValue = 0;

  void _incrementRegister() async {
    await api.incrementRegister();
    int newValue = await api.getRegisterA();
    setState(() {
      _registerValue = newValue;
    });
  }

  @override
  Widget build(BuildContext context) {
    return Center(
      child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: <Widget>[
          Text(
            'Register A: $_registerValue',
            style: TextStyle(fontSize: 24),
          ),
          SizedBox(height: 20),
          ElevatedButton(
            onPressed: _incrementRegister,
            child: Text('Increment Register'),
          ),
        ],
      ),
    );
  }
}
