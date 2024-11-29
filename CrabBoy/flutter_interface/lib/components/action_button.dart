import 'package:flutter/material.dart';

class ActionButton extends StatefulWidget {
  const ActionButton({
    Key? key,
    required this.iconPath,
    required this.pressedIconPath,
    required this.onTapDown,
    required this.onTapUp,
    required this.onTapCancel,
    this.width = 100,
    this.height = 100,
    this.disabled = false,
  }) : super(key: key);

  final String iconPath;
  final String pressedIconPath;
  final VoidCallback onTapDown;
  final VoidCallback onTapUp;
  final VoidCallback onTapCancel;
  final double width;
  final double height;
  final bool disabled;

  @override
  _ActionButtonState createState() => _ActionButtonState();
}

class _ActionButtonState extends State<ActionButton> {
  bool _isPressed = false;

  void _handleTapDown() {
    if (widget.disabled) return;

    setState(() {
      _isPressed = true;
    });
    widget.onTapDown();
  }

  void _handleTapUp() {
    if (widget.disabled) return;

    setState(() {
      _isPressed = false;
    });
    widget.onTapUp();
  }

  void _handleTapCancel() {
    if (widget.disabled) return;

    setState(() {
      _isPressed = false;
    });
    widget.onTapCancel();
  }

  @override
  Widget build(BuildContext context) {
    return GestureDetector(
      onTapDown: (_) => _handleTapDown(),
      onTapUp: (_) => _handleTapUp(),
      onTapCancel: _handleTapCancel,
      child: SizedBox(
        width: widget.width,
        height: widget.height,
        child: Image.asset(
          _isPressed ? widget.pressedIconPath : widget.iconPath,
          width: widget.width,
          height: widget.height,
          fit: BoxFit.fill,
        ),
      ),
    );
  }
}
