import 'dart:io';
import 'dart:typed_data';
import 'package:auto_route/auto_route.dart';
import 'package:flutter/material.dart';
import 'package:file_picker/file_picker.dart';
import 'package:flutter_interface/router/app_router.dart';
import 'package:path_provider/path_provider.dart';
import 'package:shared_preferences/shared_preferences.dart';

class GamesPage extends StatefulWidget {
  @override
  _GamesPageState createState() => _GamesPageState();
}

class _GamesPageState extends State<GamesPage> {
  List<String> _gameFiles = [];

  @override
  void initState() {
    super.initState();
    _loadGameFiles();
  }

  Future<void> _loadGameFiles() async {
    SharedPreferences prefs = await SharedPreferences.getInstance();
    List<String>? storedFiles = prefs.getStringList('gameFiles');
    if (storedFiles != null) {
      setState(() {
        _gameFiles = storedFiles;
      });
    }
  }

  Future<void> _saveGameFiles() async {
    SharedPreferences prefs = await SharedPreferences.getInstance();
    prefs.setStringList('gameFiles', _gameFiles);
  }

  Future<String> _getRomStoragePath() async {
    final directory = await getApplicationDocumentsDirectory();
    return directory.path;
  }

  Future<void> _pickAndSaveGameFile() async {
    FilePickerResult? result = await FilePicker.platform.pickFiles(
      type: FileType.any,
    );

    if (result != null) {
      String? filePath = result.files.single.path;
      if (filePath != null && !_gameFiles.contains(filePath)) {
        String savedFilePath = await _saveRomToInternalStorage(filePath);
        setState(() {
          _gameFiles.add(savedFilePath);
        });
        _saveGameFiles();
      }
    } else {
      print('No file selected');
    }
  }

  Future<String> _saveRomToInternalStorage(String filePath) async {
    final storagePath = await _getRomStoragePath();
    final fileName = filePath.split('/').last;
    final newFilePath = '$storagePath/$fileName';
    await File(filePath).copy(newFilePath);
    return newFilePath;
  }

  Future<void> _openGame(String gamePath) async {
    try {
      File romFile = File(gamePath);
      Uint8List romData = await romFile.readAsBytes();
      String gameName = _formatGameName(gamePath);

      Uint8List? ramData = await _loadGameRam(gameName);

      context.router.push(
          GameRoute(romData: romData, gameName: gameName, ramData: ramData));
    } catch (e) {
      print("Błąd podczas odczytu ROM: $e");
    }
  }

  Future<Uint8List?> _loadGameRam(String gameName) async {
    final storagePath = await _getRomStoragePath();
    final ramFilePath = '$storagePath/$gameName.sav';

    if (await File(ramFilePath).exists()) {
      return await File(ramFilePath).readAsBytes();
    }
    return null;
  }

  String _formatGameName(String filePath) {
    String fileName = filePath.split('/').last;
    if (fileName.endsWith('.gb')) {
      fileName = fileName.replaceAll('.gb', '');
    }
    return fileName.toUpperCase();
  }

  Future<void> _deleteGame(String gamePath) async {
    try {
      File gameFile = File(gamePath);
      if (await gameFile.exists()) {
        await gameFile.delete();
      }

      String gameName = _formatGameName(gamePath);
      final storagePath = await _getRomStoragePath();
      String saveFilePath = '$storagePath/$gameName.sav';

      File saveFile = File(saveFilePath);
      if (await saveFile.exists()) {
        await saveFile.delete();
      }

      setState(() {
        _gameFiles.remove(gamePath);
      });
      await _saveGameFiles();

      ScaffoldMessenger.of(context).showSnackBar(SnackBar(
        content: Text('Game and save data deleted successfully.'),
      ));
    } catch (e) {
      print("Error deleting game: $e");
      ScaffoldMessenger.of(context).showSnackBar(SnackBar(
        content: Text('Error deleting game.'),
      ));
    }
  }

  Future<void> _confirmAndDeleteGame(String gamePath) async {
    showDialog(
      context: context,
      builder: (BuildContext context) {
        return AlertDialog(
          title: Text('Confirm Deletion'),
          content: Text(
              'Are you sure you want to delete this game and its save data?'),
          actions: [
            TextButton(
              onPressed: () {
                Navigator.of(context).pop();
              },
              child: Text('Cancel'),
            ),
            TextButton(
              onPressed: () async {
                Navigator.of(context).pop();
                await _deleteGame(gamePath);
              },
              child: Text('Delete'),
            ),
          ],
        );
      },
    );
  }

  @override
  Widget build(BuildContext context) {
    final screenWidth = MediaQuery.of(context).size.width;
    final screenHeight = MediaQuery.of(context).size.height;
    bool isLandscape =
        MediaQuery.of(context).orientation == Orientation.landscape;
    return Scaffold(
      extendBodyBehindAppBar: true,
      appBar: AppBar(
        backgroundColor: Colors.transparent,
        elevation: 0,
        iconTheme: IconThemeData(color: Colors.white),
        automaticallyImplyLeading: true,
        centerTitle: true,
        title: Image.asset(
          'assets/images/games.png',
          height: isLandscape ? screenWidth * 0.06 : screenHeight * 0.06,
          fit: BoxFit.contain,
        ),
      ),
      body: Stack(
        children: [
          Positioned.fill(
            child: Image.asset(
              'assets/backgrounds/app_background.png',
              fit: BoxFit.cover,
            ),
          ),
          Positioned.fill(
            child: Column(
              children: [
                SizedBox(
                    height:
                        isLandscape ? screenWidth * 0.01 : screenHeight * 0.01),
                Expanded(
                  child: _gameFiles.isEmpty
                      ? Center(
                          child: Image.asset(
                            'assets/images/nogamesadded.png',
                            height: isLandscape
                                ? screenWidth * 0.03
                                : screenHeight * 0.03,
                            fit: BoxFit.contain,
                          ),
                        )
                      : ListView.builder(
                          itemCount: _gameFiles.length,
                          itemBuilder: (context, index) {
                            return ListTile(
                              contentPadding:
                                  EdgeInsets.symmetric(horizontal: 16.0),
                              leading: Icon(Icons.videogame_asset,
                                  color: Colors.white),
                              title: Text(
                                _formatGameName(_gameFiles[index]),
                                style: TextStyle(
                                  color: Colors.white,
                                  fontSize: 25,
                                  fontWeight: FontWeight.bold,
                                ),
                              ),
                              onTap: () => _openGame(_gameFiles[index]),
                              onLongPress: () =>
                                  _confirmAndDeleteGame(_gameFiles[index]),
                            );
                          },
                        ),
                ),
              ],
            ),
          ),
        ],
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: _pickAndSaveGameFile,
        child: Icon(Icons.add),
      ),
    );
  }
}
