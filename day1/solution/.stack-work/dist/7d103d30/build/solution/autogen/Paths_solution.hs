{-# LANGUAGE CPP #-}
{-# LANGUAGE NoRebindableSyntax #-}
{-# OPTIONS_GHC -fno-warn-missing-import-lists #-}
module Paths_solution (
    version,
    getBinDir, getLibDir, getDynLibDir, getDataDir, getLibexecDir,
    getDataFileName, getSysconfDir
  ) where

import qualified Control.Exception as Exception
import Data.Version (Version(..))
import System.Environment (getEnv)
import Prelude

#if defined(VERSION_base)

#if MIN_VERSION_base(4,0,0)
catchIO :: IO a -> (Exception.IOException -> IO a) -> IO a
#else
catchIO :: IO a -> (Exception.Exception -> IO a) -> IO a
#endif

#else
catchIO :: IO a -> (Exception.IOException -> IO a) -> IO a
#endif
catchIO = Exception.catch

version :: Version
version = Version [0,1,0,0] []
bindir, libdir, dynlibdir, datadir, libexecdir, sysconfdir :: FilePath

bindir     = "C:\\Users\\Alex\\Desktop\\Code\\Advent of Code\\2018\\day1\\solution\\.stack-work\\install\\ad603a72\\bin"
libdir     = "C:\\Users\\Alex\\Desktop\\Code\\Advent of Code\\2018\\day1\\solution\\.stack-work\\install\\ad603a72\\lib\\x86_64-windows-ghc-8.4.4\\solution-0.1.0.0-4Be3ddOMUc5KboR1ouGHM4-solution"
dynlibdir  = "C:\\Users\\Alex\\Desktop\\Code\\Advent of Code\\2018\\day1\\solution\\.stack-work\\install\\ad603a72\\lib\\x86_64-windows-ghc-8.4.4"
datadir    = "C:\\Users\\Alex\\Desktop\\Code\\Advent of Code\\2018\\day1\\solution\\.stack-work\\install\\ad603a72\\share\\x86_64-windows-ghc-8.4.4\\solution-0.1.0.0"
libexecdir = "C:\\Users\\Alex\\Desktop\\Code\\Advent of Code\\2018\\day1\\solution\\.stack-work\\install\\ad603a72\\libexec\\x86_64-windows-ghc-8.4.4\\solution-0.1.0.0"
sysconfdir = "C:\\Users\\Alex\\Desktop\\Code\\Advent of Code\\2018\\day1\\solution\\.stack-work\\install\\ad603a72\\etc"

getBinDir, getLibDir, getDynLibDir, getDataDir, getLibexecDir, getSysconfDir :: IO FilePath
getBinDir = catchIO (getEnv "solution_bindir") (\_ -> return bindir)
getLibDir = catchIO (getEnv "solution_libdir") (\_ -> return libdir)
getDynLibDir = catchIO (getEnv "solution_dynlibdir") (\_ -> return dynlibdir)
getDataDir = catchIO (getEnv "solution_datadir") (\_ -> return datadir)
getLibexecDir = catchIO (getEnv "solution_libexecdir") (\_ -> return libexecdir)
getSysconfDir = catchIO (getEnv "solution_sysconfdir") (\_ -> return sysconfdir)

getDataFileName :: FilePath -> IO FilePath
getDataFileName name = do
  dir <- getDataDir
  return (dir ++ "\\" ++ name)
