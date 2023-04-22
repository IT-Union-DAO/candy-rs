let upstream = https://github.com/dfinity/vessel-package-set/releases/download/mo-0.8.3-20230224/package-set.dhall sha256:df71395fc48d1a0f41c0b49a0a6e0490d0b52641a86a9d51651bf2622e87fbff
let Package =
    { name : Text, version : Text, repo : Text, dependencies : List Text }

let
  -- This is where you can add your own packages to the package-set
  additions =
    [
    { name = "candid"
          , version = "v1.0.1"
          , repo = "https://github.com/gekctek/motoko_candid"
          , dependencies = ["xtendedNumbers", "base"] : List Text
          },
    { name = "candy",
      repo = "https://github.com/icdevs/candy_library",
      version = "0.2.0",
      dependencies = ["base"]
    },
    { name = "map"
      , repo = "https://github.com/ZhenyaUsenko/motoko-hash-map"
      , version = "v7.0.0"
      , dependencies = [ "base"]
      },
      { name = "stablebuffer"
        , repo = "https://github.com/skilesare/StableBuffer"
        , version = "v0.2.0"
        , dependencies = [ "base"]
        },
        { name = "xtendedNumbers"
              , version = "v1.0.2"
              , repo = "https://github.com/gekctek/motoko_numbers"
              , dependencies = [] : List Text
              }
    ] : List Package

let
  {- This is where you can override existing packages in the package-set

     For example, if you wanted to use version `v2.0.0` of the foo library:
     let overrides = [
         { name = "foo"
         , version = "v2.0.0"
         , repo = "https://github.com/bar/foo"
         , dependencies = [] : List Text
         }
     ]
  -}
  overrides =
    [] : List Package

in  upstream # additions # overrides
