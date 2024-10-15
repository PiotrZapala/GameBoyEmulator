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
        child: GamePage(gamePath: args.gamePath),
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
  GameRoute({required String gamePath})
      : super(
          GameRoute.name,
          path: '/game-page',
          args: GameRouteArgs(gamePath: gamePath),
        );

  static const String name = 'GameRoute';
}

class GameRouteArgs {
  const GameRouteArgs({required this.gamePath});

  final String gamePath;

  @override
  String toString() {
    return 'GameRouteArgs{gamePath: $gamePath}';
  }
}
