extern crate js_parser;
#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

use js_parser::lexer::token::{Number, Token};
use js_parser::lexer::Lexer;

// Test real world example from React build script
// https://raw.githubusercontent.com/facebook/react/master/scripts/release/build.js
#[test]
fn react_build_script() {
    let (unparsed, parsed) = Lexer::lex_tokens(
        r#"'use strict';

const {exec} = require('child_process');

// Follows the steps outlined in github.com/facebook/react/issues/10620
const run = async () => {
  const chalk = require('chalk');
  const logUpdate = require('log-update');
  const {getPublicPackages, getPackages} = require('./utils');

  const addGitTag = require('./build-commands/add-git-tag');
  const buildArtifacts = require('./build-commands/build-artifacts');
  const checkCircleCiStatus = require('./build-commands/check-circle-ci-status');
  const checkEnvironmentVariables = require('./build-commands/check-environment-variables');
  const checkNpmPermissions = require('./build-commands/check-npm-permissions');
  const checkPackageDependencies = require('./build-commands/check-package-dependencies');
  const checkUncommittedChanges = require('./build-commands/check-uncommitted-changes');
  const installYarnDependencies = require('./build-commands/install-yarn-dependencies');
  const parseBuildParameters = require('./build-commands/parse-build-parameters');
  const printPostBuildSummary = require('./build-commands/print-post-build-summary');
  const runAutomatedTests = require('./build-commands/run-automated-tests');
  const runAutomatedBundleTests = require('./build-commands/run-automated-bundle-tests');
  const updateGit = require('./build-commands/update-git');
  const updateNoopRendererDependencies = require('./build-commands/update-noop-renderer-dependencies');
  const updatePackageVersions = require('./build-commands/update-package-versions');
  const updateYarnDependencies = require('./build-commands/update-yarn-dependencies');
  const validateVersion = require('./build-commands/validate-version');

  try {
    const params = parseBuildParameters();
    params.packages = getPublicPackages();

    await checkEnvironmentVariables(params);
    await validateVersion(params);
    await checkUncommittedChanges(params);
    await checkNpmPermissions(params);
    await updateGit(params);
    await checkCircleCiStatus(params);
    await installYarnDependencies(params);
    await checkPackageDependencies(params);
    await updateYarnDependencies(params);
    await runAutomatedTests(params);
    // Also update NPM dependencies for private packages (e.g. react-native-renderer)
    // Even though we don't publish these to NPM,
    // mismatching dependencies can cause `yarn install` to install duplicate packages.
    await updatePackageVersions({
      ...params,
      packages: getPackages(),
    });
    await updateNoopRendererDependencies(params);
    await buildArtifacts(params);
    await runAutomatedBundleTests(params);
    await addGitTag(params);
    await printPostBuildSummary(params);
  } catch (error) {
    logUpdate.clear();

    const message = error.message.trim().replace('\n');
    const stack = error.stack.replace(error.message, '');

    console.log(
      "chalk.bgRed.white(' ERROR ')"
    );

    process.exit(1);
  }
};

// Install (or update) release script dependencies before proceeding.
// This needs to be done before we require() the first NPM module.
exec('yarn install', {cwd: __dirname}, (error, stdout, stderr) => {
  if (error) {
    console.error(error);
    process.exit(1);
  } else {
    run();
  }
});

        "#,
    ).unwrap();
    assert_eq!(
        unparsed.len(),
        0,
        "unparsed should be empty, contains {}",
        unparsed
    );
    assert_eq!(
        parsed,
        vec![
            Token::StringLiteral(String::from("use strict")),
            Token::Semicolon,
            Token::LF,
            Token::LF,
            Token::KConst,
            Token::LBrace,
            Token::IdentifierName(String::from("exec")),
            Token::RBrace,
            Token::Assign,
            Token::IdentifierName(String::from("require")),
            Token::LRound,
            Token::StringLiteral(String::from("child_process")),
            Token::RRound,
            Token::Semicolon,
            Token::LF,
            Token::LF,
            Token::LF,
            Token::KConst,
            Token::IdentifierName(String::from("run")),
            Token::Assign,
            Token::KAsync,
            Token::LRound,
            Token::RRound,
            Token::EqualArrow,
            Token::LBrace,
            Token::LF,
            Token::KConst,
            Token::IdentifierName(String::from("chalk")),
            Token::Assign,
            Token::IdentifierName(String::from("require")),
            Token::LRound,
            Token::StringLiteral(String::from("chalk")),
            Token::RRound,
            Token::Semicolon,
            Token::LF,
            Token::KConst,
            Token::IdentifierName(String::from("logUpdate")),
            Token::Assign,
            Token::IdentifierName(String::from("require")),
            Token::LRound,
            Token::StringLiteral(String::from("log-update")),
            Token::RRound,
            Token::Semicolon,
            Token::LF,
            Token::KConst,
            Token::LBrace,
            Token::IdentifierName(String::from("getPublicPackages")),
            Token::Comma,
            Token::IdentifierName(String::from("getPackages")),
            Token::RBrace,
            Token::Assign,
            Token::IdentifierName(String::from("require")),
            Token::LRound,
            Token::StringLiteral(String::from("./utils")),
            Token::RRound,
            Token::Semicolon,
            Token::LF,
            Token::LF,
            Token::KConst,
            Token::IdentifierName(String::from("addGitTag")),
            Token::Assign,
            Token::IdentifierName(String::from("require")),
            Token::LRound,
            Token::StringLiteral(String::from("./build-commands/add-git-tag")),
            Token::RRound,
            Token::Semicolon,
            Token::LF,
            Token::KConst,
            Token::IdentifierName(String::from("buildArtifacts")),
            Token::Assign,
            Token::IdentifierName(String::from("require")),
            Token::LRound,
            Token::StringLiteral(String::from("./build-commands/build-artifacts")),
            Token::RRound,
            Token::Semicolon,
            Token::LF,
            Token::KConst,
            Token::IdentifierName(String::from("checkCircleCiStatus")),
            Token::Assign,
            Token::IdentifierName(String::from("require")),
            Token::LRound,
            Token::StringLiteral(String::from("./build-commands/check-circle-ci-status")),
            Token::RRound,
            Token::Semicolon,
            Token::LF,
            Token::KConst,
            Token::IdentifierName(String::from("checkEnvironmentVariables")),
            Token::Assign,
            Token::IdentifierName(String::from("require")),
            Token::LRound,
            Token::StringLiteral(String::from("./build-commands/check-environment-variables")),
            Token::RRound,
            Token::Semicolon,
            Token::LF,
            Token::KConst,
            Token::IdentifierName(String::from("checkNpmPermissions")),
            Token::Assign,
            Token::IdentifierName(String::from("require")),
            Token::LRound,
            Token::StringLiteral(String::from("./build-commands/check-npm-permissions")),
            Token::RRound,
            Token::Semicolon,
            Token::LF,
            Token::KConst,
            Token::IdentifierName(String::from("checkPackageDependencies")),
            Token::Assign,
            Token::IdentifierName(String::from("require")),
            Token::LRound,
            Token::StringLiteral(String::from("./build-commands/check-package-dependencies")),
            Token::RRound,
            Token::Semicolon,
            Token::LF,
            Token::KConst,
            Token::IdentifierName(String::from("checkUncommittedChanges")),
            Token::Assign,
            Token::IdentifierName(String::from("require")),
            Token::LRound,
            Token::StringLiteral(String::from("./build-commands/check-uncommitted-changes")),
            Token::RRound,
            Token::Semicolon,
            Token::LF,
            Token::KConst,
            Token::IdentifierName(String::from("installYarnDependencies")),
            Token::Assign,
            Token::IdentifierName(String::from("require")),
            Token::LRound,
            Token::StringLiteral(String::from("./build-commands/install-yarn-dependencies")),
            Token::RRound,
            Token::Semicolon,
            Token::LF,
            Token::KConst,
            Token::IdentifierName(String::from("parseBuildParameters")),
            Token::Assign,
            Token::IdentifierName(String::from("require")),
            Token::LRound,
            Token::StringLiteral(String::from("./build-commands/parse-build-parameters")),
            Token::RRound,
            Token::Semicolon,
            Token::LF,
            Token::KConst,
            Token::IdentifierName(String::from("printPostBuildSummary")),
            Token::Assign,
            Token::IdentifierName(String::from("require")),
            Token::LRound,
            Token::StringLiteral(String::from("./build-commands/print-post-build-summary")),
            Token::RRound,
            Token::Semicolon,
            Token::LF,
            Token::KConst,
            Token::IdentifierName(String::from("runAutomatedTests")),
            Token::Assign,
            Token::IdentifierName(String::from("require")),
            Token::LRound,
            Token::StringLiteral(String::from("./build-commands/run-automated-tests")),
            Token::RRound,
            Token::Semicolon,
            Token::LF,
            Token::KConst,
            Token::IdentifierName(String::from("runAutomatedBundleTests")),
            Token::Assign,
            Token::IdentifierName(String::from("require")),
            Token::LRound,
            Token::StringLiteral(String::from("./build-commands/run-automated-bundle-tests")),
            Token::RRound,
            Token::Semicolon,
            Token::LF,
            Token::KConst,
            Token::IdentifierName(String::from("updateGit")),
            Token::Assign,
            Token::IdentifierName(String::from("require")),
            Token::LRound,
            Token::StringLiteral(String::from("./build-commands/update-git")),
            Token::RRound,
            Token::Semicolon,
            Token::LF,
            Token::KConst,
            Token::IdentifierName(String::from("updateNoopRendererDependencies")),
            Token::Assign,
            Token::IdentifierName(String::from("require")),
            Token::LRound,
            Token::StringLiteral(String::from(
                "./build-commands/update-noop-renderer-dependencies"
            )),
            Token::RRound,
            Token::Semicolon,
            Token::LF,
            Token::KConst,
            Token::IdentifierName(String::from("updatePackageVersions")),
            Token::Assign,
            Token::IdentifierName(String::from("require")),
            Token::LRound,
            Token::StringLiteral(String::from("./build-commands/update-package-versions")),
            Token::RRound,
            Token::Semicolon,
            Token::LF,
            Token::KConst,
            Token::IdentifierName(String::from("updateYarnDependencies")),
            Token::Assign,
            Token::IdentifierName(String::from("require")),
            Token::LRound,
            Token::StringLiteral(String::from("./build-commands/update-yarn-dependencies")),
            Token::RRound,
            Token::Semicolon,
            Token::LF,
            Token::KConst,
            Token::IdentifierName(String::from("validateVersion")),
            Token::Assign,
            Token::IdentifierName(String::from("require")),
            Token::LRound,
            Token::StringLiteral(String::from("./build-commands/validate-version")),
            Token::RRound,
            Token::Semicolon,
            Token::LF,
            Token::LF,
            Token::KTry,
            Token::LBrace,
            Token::LF,
            Token::KConst,
            Token::IdentifierName(String::from("params")),
            Token::Assign,
            Token::IdentifierName(String::from("parseBuildParameters")),
            Token::LRound,
            Token::RRound,
            Token::Semicolon,
            Token::LF,
            Token::IdentifierName(String::from("params")),
            Token::Dot,
            Token::IdentifierName(String::from("packages")),
            Token::Assign,
            Token::IdentifierName(String::from("getPublicPackages")),
            Token::LRound,
            Token::RRound,
            Token::Semicolon,
            Token::LF,
            Token::LF,
            Token::KAwait,
            Token::IdentifierName(String::from("checkEnvironmentVariables")),
            Token::LRound,
            Token::IdentifierName(String::from("params")),
            Token::RRound,
            Token::Semicolon,
            Token::LF,
            Token::KAwait,
            Token::IdentifierName(String::from("validateVersion")),
            Token::LRound,
            Token::IdentifierName(String::from("params")),
            Token::RRound,
            Token::Semicolon,
            Token::LF,
            Token::KAwait,
            Token::IdentifierName(String::from("checkUncommittedChanges")),
            Token::LRound,
            Token::IdentifierName(String::from("params")),
            Token::RRound,
            Token::Semicolon,
            Token::LF,
            Token::KAwait,
            Token::IdentifierName(String::from("checkNpmPermissions")),
            Token::LRound,
            Token::IdentifierName(String::from("params")),
            Token::RRound,
            Token::Semicolon,
            Token::LF,
            Token::KAwait,
            Token::IdentifierName(String::from("updateGit")),
            Token::LRound,
            Token::IdentifierName(String::from("params")),
            Token::RRound,
            Token::Semicolon,
            Token::LF,
            Token::KAwait,
            Token::IdentifierName(String::from("checkCircleCiStatus")),
            Token::LRound,
            Token::IdentifierName(String::from("params")),
            Token::RRound,
            Token::Semicolon,
            Token::LF,
            Token::KAwait,
            Token::IdentifierName(String::from("installYarnDependencies")),
            Token::LRound,
            Token::IdentifierName(String::from("params")),
            Token::RRound,
            Token::Semicolon,
            Token::LF,
            Token::KAwait,
            Token::IdentifierName(String::from("checkPackageDependencies")),
            Token::LRound,
            Token::IdentifierName(String::from("params")),
            Token::RRound,
            Token::Semicolon,
            Token::LF,
            Token::KAwait,
            Token::IdentifierName(String::from("updateYarnDependencies")),
            Token::LRound,
            Token::IdentifierName(String::from("params")),
            Token::RRound,
            Token::Semicolon,
            Token::LF,
            Token::KAwait,
            Token::IdentifierName(String::from("runAutomatedTests")),
            Token::LRound,
            Token::IdentifierName(String::from("params")),
            Token::RRound,
            Token::Semicolon,
            Token::LF,
            Token::LF,
            Token::LF,
            Token::LF,
            Token::KAwait,
            Token::IdentifierName(String::from("updatePackageVersions")),
            Token::LRound,
            Token::LBrace,
            Token::LF,
            Token::TripleDot,
            Token::IdentifierName(String::from("params")),
            Token::Comma,
            Token::LF,
            Token::IdentifierName(String::from("packages")),
            Token::Colon,
            Token::IdentifierName(String::from("getPackages")),
            Token::LRound,
            Token::RRound,
            Token::Comma,
            Token::LF,
            Token::RBrace,
            Token::RRound,
            Token::Semicolon,
            Token::LF,
            Token::KAwait,
            Token::IdentifierName(String::from("updateNoopRendererDependencies")),
            Token::LRound,
            Token::IdentifierName(String::from("params")),
            Token::RRound,
            Token::Semicolon,
            Token::LF,
            Token::KAwait,
            Token::IdentifierName(String::from("buildArtifacts")),
            Token::LRound,
            Token::IdentifierName(String::from("params")),
            Token::RRound,
            Token::Semicolon,
            Token::LF,
            Token::KAwait,
            Token::IdentifierName(String::from("runAutomatedBundleTests")),
            Token::LRound,
            Token::IdentifierName(String::from("params")),
            Token::RRound,
            Token::Semicolon,
            Token::LF,
            Token::KAwait,
            Token::IdentifierName(String::from("addGitTag")),
            Token::LRound,
            Token::IdentifierName(String::from("params")),
            Token::RRound,
            Token::Semicolon,
            Token::LF,
            Token::KAwait,
            Token::IdentifierName(String::from("printPostBuildSummary")),
            Token::LRound,
            Token::IdentifierName(String::from("params")),
            Token::RRound,
            Token::Semicolon,
            Token::LF,
            Token::RBrace,
            Token::KCatch,
            Token::LRound,
            Token::IdentifierName(String::from("error")),
            Token::RRound,
            Token::LBrace,
            Token::LF,
            Token::IdentifierName(String::from("logUpdate")),
            Token::Dot,
            Token::IdentifierName(String::from("clear")),
            Token::LRound,
            Token::RRound,
            Token::Semicolon,
            Token::LF,
            Token::LF,
            Token::KConst,
            Token::IdentifierName(String::from("message")),
            Token::Assign,
            Token::IdentifierName(String::from("error")),
            Token::Dot,
            Token::IdentifierName(String::from("message")),
            Token::Dot,
            Token::IdentifierName(String::from("trim")),
            Token::LRound,
            Token::RRound,
            Token::Dot,
            Token::IdentifierName(String::from("replace")),
            Token::LRound,
            Token::StringLiteral(String::from("\n")),
            Token::RRound,
            Token::Semicolon,
            Token::LF,
            Token::KConst,
            Token::IdentifierName(String::from("stack")),
            Token::Assign,
            Token::IdentifierName(String::from("error")),
            Token::Dot,
            Token::IdentifierName(String::from("stack")),
            Token::Dot,
            Token::IdentifierName(String::from("replace")),
            Token::LRound,
            Token::IdentifierName(String::from("error")),
            Token::Dot,
            Token::IdentifierName(String::from("message")),
            Token::Comma,
            Token::StringLiteral(String::new()),
            Token::RRound,
            Token::Semicolon,
            Token::LF,
            Token::LF,
            Token::IdentifierName(String::from("console")),
            Token::Dot,
            Token::IdentifierName(String::from("log")),
            Token::LRound,
            Token::LF,
            Token::StringLiteral(String::from("chalk.bgRed.white(\' ERROR \')")),
            Token::LF,
            Token::RRound,
            Token::Semicolon,
            Token::LF,
            Token::LF,
            Token::IdentifierName(String::from("process")),
            Token::Dot,
            Token::IdentifierName(String::from("exit")),
            Token::LRound,
            Token::NumericLiteral(Number::new(1, 0, 1, 10)),
            Token::RRound,
            Token::Semicolon,
            Token::LF,
            Token::RBrace,
            Token::LF,
            Token::RBrace,
            Token::Semicolon,
            Token::LF,
            Token::LF,
            Token::LF,
            Token::LF,
            Token::IdentifierName(String::from("exec")),
            Token::LRound,
            Token::StringLiteral(String::from("yarn install")),
            Token::Comma,
            Token::LBrace,
            Token::IdentifierName(String::from("cwd")),
            Token::Colon,
            Token::IdentifierName(String::from("__dirname")),
            Token::RBrace,
            Token::Comma,
            Token::LRound,
            Token::IdentifierName(String::from("error")),
            Token::Comma,
            Token::IdentifierName(String::from("stdout")),
            Token::Comma,
            Token::IdentifierName(String::from("stderr")),
            Token::RRound,
            Token::EqualArrow,
            Token::LBrace,
            Token::LF,
            Token::KIf,
            Token::LRound,
            Token::IdentifierName(String::from("error")),
            Token::RRound,
            Token::LBrace,
            Token::LF,
            Token::IdentifierName(String::from("console")),
            Token::Dot,
            Token::IdentifierName(String::from("error")),
            Token::LRound,
            Token::IdentifierName(String::from("error")),
            Token::RRound,
            Token::Semicolon,
            Token::LF,
            Token::IdentifierName(String::from("process")),
            Token::Dot,
            Token::IdentifierName(String::from("exit")),
            Token::LRound,
            Token::NumericLiteral(Number::new(1, 0, 1, 10)),
            Token::RRound,
            Token::Semicolon,
            Token::LF,
            Token::RBrace,
            Token::KElse,
            Token::LBrace,
            Token::LF,
            Token::IdentifierName(String::from("run")),
            Token::LRound,
            Token::RRound,
            Token::Semicolon,
            Token::LF,
            Token::RBrace,
            Token::LF,
            Token::RBrace,
            Token::RRound,
            Token::Semicolon,
            Token::LF,
            Token::LF,
            Token::EOF
        ]
    );
}