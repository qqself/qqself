// swift-tools-version:5.6
import PackageDescription

let package = Package(
  name: "qqselfCoreLib",
  platforms: [
    .iOS(.v15)
  ],
  products: [
    .library(
      name: "qqselfCoreLib",
      targets: ["qqselfCoreLib"])
  ],
  dependencies: [],
  targets: [
    .target(
      name: "qqselfCoreLib",
      dependencies: [
        .byName(name: "qqselfCore")
      ],
      path: "Sources/"
    ),
    .binaryTarget(
      name: "qqselfCore",
      path: "artifacts/qqselfCore.xcframework"
    ),
  ]
)
