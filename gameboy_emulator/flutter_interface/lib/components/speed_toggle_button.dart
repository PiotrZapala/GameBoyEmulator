import 'package:flutter/material.dart';

class SpeedToggleButton extends StatefulWidget {
  final bool isDoubleSpeed;
  final VoidCallback toggleSpeed;
  final double width;
  final double height;

  const SpeedToggleButton({
    Key? key,
    this.width = 100,
    this.height = 100,
    required this.isDoubleSpeed,
    required this.toggleSpeed,
  }) : super(key: key);

  @override
  _SpeedToggleButtonState createState() => _SpeedToggleButtonState();
}

class _SpeedToggleButtonState extends State<SpeedToggleButton> {
  @override
  Widget build(BuildContext context) {
    return GestureDetector(
      onTap: widget.toggleSpeed,
      child: SizedBox(
        width: widget.width,
        height: widget.height,
        child: Image.asset(
          widget.isDoubleSpeed
              ? 'assets/buttons/x2_hover.png'
              : 'assets/buttons/x2.png',
          width: widget.width,
          height: widget.height,
          fit: BoxFit.fill,
        ),
      ),
    );
  }
}
