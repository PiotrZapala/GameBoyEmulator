// **************************************************************************
// AutoRouteGenerator
// **************************************************************************

// GENERATED CODE - DO NOT MODIFY BY HAND

// **************************************************************************
// AutoRouteGenerator
// **************************************************************************
//
// ignore_for_file: type=lint

part of 'app_router.dart';

class _$AppRouter extends RootStackRouter {
  _$AppRouter([GlobalKey<NavigatorState>? navigatorKey]) : super(navigatorKey);

  @override
  final Map<String, PageFactory> pagesMap = {
    HomeRoute.name: (routeData) {
      return MaterialPageX<dynamic>(
        routeData: routeData,
        child: HomePage(),
      );
    },
    GamesRoute.name: (routeData) {
      return MaterialPageX<dynamic>(
        routeData: routeData,
        child: GamesPage(),
      );
    },
    GameRoute.name: (routeData) {
      final args = routeData.argsAs<GameRouteArgs>();
      return MaterialPageX<dynamic>(
        routeData: routeData,
        child: GamePage(
          romData: args.romData,
          gameName: args.gameName,
          ramData: args.ramData,
        ),
      );
    },
  };

  @override
  List<RouteConfig> get routes => [
        RouteConfig(
          HomeRoute.name,
          path: '/',
        ),
        RouteConfig(
          GamesRoute.name,
          path: '/games-page',
        ),
        RouteConfig(
          GameRoute.name,
          path: '/game-page',
        ),
      ];
}

/// generated route for
/// [HomePage]
class HomeRoute extends PageRouteInfo<void> {
  const HomeRoute()
      : super(
          HomeRoute.name,
          path: '/',
        );

  static const String name = 'HomeRoute';
}

/// generated route for
/// [GamesPage]
class GamesRoute extends PageRouteInfo<void> {
  const GamesRoute()
      : super(
          GamesRoute.name,
          path: '/games-page',
        );

  static const String name = 'GamesRoute';
}

/// generated route for
/// [GamePage]
class GameRoute extends PageRouteInfo<GameRouteArgs> {
  GameRoute({
    required Uint8List romData,
    required String gameName,
    required Uint8List? ramData,
  }) : super(
          GameRoute.name,
          path: '/game-page',
          args: GameRouteArgs(
            romData: romData,
            gameName: gameName,
            ramData: ramData,
          ),
        );

  static const String name = 'GameRoute';
}

class GameRouteArgs {
  const GameRouteArgs({
    required this.romData,
    required this.gameName,
    required this.ramData,
  });

  final Uint8List romData;

  final String gameName;

  final Uint8List? ramData;

  @override
  String toString() {
    return 'GameRouteArgs{romData: $romData, gameName: $gameName, ramData: $ramData}';
  }
}
