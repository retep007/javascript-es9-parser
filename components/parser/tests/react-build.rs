extern crate javascript_lexer;
#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

use javascript_lexer::{
    internship,
    token::{Number, Token},
    Lexer,
};

// Test real world example from React build script
// https://raw.githubusercontent.com/facebook/react/master/scripts/release/build.js
#[test]
fn react_build_script() {
    let parsed = Lexer::lex_tokens(
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
        parsed,
        vec![
            Token::StringLiteral(String::from("use strict")),
            Token::Semicolon,
            Token::LineTerminator,
            Token::LineTerminator,
            Token::KConst,
            Token::LCurly,
            Token::IdentifierName(internship::IStr::new("exec")),
            Token::RCurly,
            Token::Assign,
            Token::IdentifierName(internship::IStr::new("require")),
            Token::LRound,
            Token::StringLiteral(String::from("child_process")),
            Token::RRound,
            Token::Semicolon,
            Token::LineTerminator,
            Token::LineTerminator,
            Token::LineTerminator,
            Token::KConst,
            Token::IdentifierName(internship::IStr::new("run")),
            Token::Assign,
            Token::KAsync,
            Token::LRound,
            Token::RRound,
            Token::AssignBigger,
            Token::LCurly,
            Token::LineTerminator,
            Token::KConst,
            Token::IdentifierName(internship::IStr::new("chalk")),
            Token::Assign,
            Token::IdentifierName(internship::IStr::new("require")),
            Token::LRound,
            Token::StringLiteral(String::from("chalk")),
            Token::RRound,
            Token::Semicolon,
            Token::LineTerminator,
            Token::KConst,
            Token::IdentifierName(internship::IStr::new("logUpdate")),
            Token::Assign,
            Token::IdentifierName(internship::IStr::new("require")),
            Token::LRound,
            Token::StringLiteral(String::from("log-update")),
            Token::RRound,
            Token::Semicolon,
            Token::LineTerminator,
            Token::KConst,
            Token::LCurly,
            Token::IdentifierName(internship::IStr::new("getPublicPackages")),
            Token::Comma,
            Token::IdentifierName(internship::IStr::new("getPackages")),
            Token::RCurly,
            Token::Assign,
            Token::IdentifierName(internship::IStr::new("require")),
            Token::LRound,
            Token::StringLiteral(String::from("./utils")),
            Token::RRound,
            Token::Semicolon,
            Token::LineTerminator,
            Token::LineTerminator,
            Token::KConst,
            Token::IdentifierName(internship::IStr::new("addGitTag")),
            Token::Assign,
            Token::IdentifierName(internship::IStr::new("require")),
            Token::LRound,
            Token::StringLiteral(String::from("./build-commands/add-git-tag")),
            Token::RRound,
            Token::Semicolon,
            Token::LineTerminator,
            Token::KConst,
            Token::IdentifierName(internship::IStr::new("buildArtifacts")),
            Token::Assign,
            Token::IdentifierName(internship::IStr::new("require")),
            Token::LRound,
            Token::StringLiteral(String::from("./build-commands/build-artifacts")),
            Token::RRound,
            Token::Semicolon,
            Token::LineTerminator,
            Token::KConst,
            Token::IdentifierName(internship::IStr::new("checkCircleCiStatus")),
            Token::Assign,
            Token::IdentifierName(internship::IStr::new("require")),
            Token::LRound,
            Token::StringLiteral(String::from("./build-commands/check-circle-ci-status")),
            Token::RRound,
            Token::Semicolon,
            Token::LineTerminator,
            Token::KConst,
            Token::IdentifierName(internship::IStr::new("checkEnvironmentVariables")),
            Token::Assign,
            Token::IdentifierName(internship::IStr::new("require")),
            Token::LRound,
            Token::StringLiteral(String::from("./build-commands/check-environment-variables")),
            Token::RRound,
            Token::Semicolon,
            Token::LineTerminator,
            Token::KConst,
            Token::IdentifierName(internship::IStr::new("checkNpmPermissions")),
            Token::Assign,
            Token::IdentifierName(internship::IStr::new("require")),
            Token::LRound,
            Token::StringLiteral(String::from("./build-commands/check-npm-permissions")),
            Token::RRound,
            Token::Semicolon,
            Token::LineTerminator,
            Token::KConst,
            Token::IdentifierName(internship::IStr::new("checkPackageDependencies")),
            Token::Assign,
            Token::IdentifierName(internship::IStr::new("require")),
            Token::LRound,
            Token::StringLiteral(String::from("./build-commands/check-package-dependencies")),
            Token::RRound,
            Token::Semicolon,
            Token::LineTerminator,
            Token::KConst,
            Token::IdentifierName(internship::IStr::new("checkUncommittedChanges")),
            Token::Assign,
            Token::IdentifierName(internship::IStr::new("require")),
            Token::LRound,
            Token::StringLiteral(String::from("./build-commands/check-uncommitted-changes")),
            Token::RRound,
            Token::Semicolon,
            Token::LineTerminator,
            Token::KConst,
            Token::IdentifierName(internship::IStr::new("installYarnDependencies")),
            Token::Assign,
            Token::IdentifierName(internship::IStr::new("require")),
            Token::LRound,
            Token::StringLiteral(String::from("./build-commands/install-yarn-dependencies")),
            Token::RRound,
            Token::Semicolon,
            Token::LineTerminator,
            Token::KConst,
            Token::IdentifierName(internship::IStr::new("parseBuildParameters")),
            Token::Assign,
            Token::IdentifierName(internship::IStr::new("require")),
            Token::LRound,
            Token::StringLiteral(String::from("./build-commands/parse-build-parameters")),
            Token::RRound,
            Token::Semicolon,
            Token::LineTerminator,
            Token::KConst,
            Token::IdentifierName(internship::IStr::new("printPostBuildSummary")),
            Token::Assign,
            Token::IdentifierName(internship::IStr::new("require")),
            Token::LRound,
            Token::StringLiteral(String::from("./build-commands/print-post-build-summary")),
            Token::RRound,
            Token::Semicolon,
            Token::LineTerminator,
            Token::KConst,
            Token::IdentifierName(internship::IStr::new("runAutomatedTests")),
            Token::Assign,
            Token::IdentifierName(internship::IStr::new("require")),
            Token::LRound,
            Token::StringLiteral(String::from("./build-commands/run-automated-tests")),
            Token::RRound,
            Token::Semicolon,
            Token::LineTerminator,
            Token::KConst,
            Token::IdentifierName(internship::IStr::new("runAutomatedBundleTests")),
            Token::Assign,
            Token::IdentifierName(internship::IStr::new("require")),
            Token::LRound,
            Token::StringLiteral(String::from("./build-commands/run-automated-bundle-tests")),
            Token::RRound,
            Token::Semicolon,
            Token::LineTerminator,
            Token::KConst,
            Token::IdentifierName(internship::IStr::new("updateGit")),
            Token::Assign,
            Token::IdentifierName(internship::IStr::new("require")),
            Token::LRound,
            Token::StringLiteral(String::from("./build-commands/update-git")),
            Token::RRound,
            Token::Semicolon,
            Token::LineTerminator,
            Token::KConst,
            Token::IdentifierName(internship::IStr::new("updateNoopRendererDependencies")),
            Token::Assign,
            Token::IdentifierName(internship::IStr::new("require")),
            Token::LRound,
            Token::StringLiteral(String::from(
                "./build-commands/update-noop-renderer-dependencies"
            )),
            Token::RRound,
            Token::Semicolon,
            Token::LineTerminator,
            Token::KConst,
            Token::IdentifierName(internship::IStr::new("updatePackageVersions")),
            Token::Assign,
            Token::IdentifierName(internship::IStr::new("require")),
            Token::LRound,
            Token::StringLiteral(String::from("./build-commands/update-package-versions")),
            Token::RRound,
            Token::Semicolon,
            Token::LineTerminator,
            Token::KConst,
            Token::IdentifierName(internship::IStr::new("updateYarnDependencies")),
            Token::Assign,
            Token::IdentifierName(internship::IStr::new("require")),
            Token::LRound,
            Token::StringLiteral(String::from("./build-commands/update-yarn-dependencies")),
            Token::RRound,
            Token::Semicolon,
            Token::LineTerminator,
            Token::KConst,
            Token::IdentifierName(internship::IStr::new("validateVersion")),
            Token::Assign,
            Token::IdentifierName(internship::IStr::new("require")),
            Token::LRound,
            Token::StringLiteral(String::from("./build-commands/validate-version")),
            Token::RRound,
            Token::Semicolon,
            Token::LineTerminator,
            Token::LineTerminator,
            Token::KTry,
            Token::LCurly,
            Token::LineTerminator,
            Token::KConst,
            Token::IdentifierName(internship::IStr::new("params")),
            Token::Assign,
            Token::IdentifierName(internship::IStr::new("parseBuildParameters")),
            Token::LRound,
            Token::RRound,
            Token::Semicolon,
            Token::LineTerminator,
            Token::IdentifierName(internship::IStr::new("params")),
            Token::Dot,
            Token::IdentifierName(internship::IStr::new("packages")),
            Token::Assign,
            Token::IdentifierName(internship::IStr::new("getPublicPackages")),
            Token::LRound,
            Token::RRound,
            Token::Semicolon,
            Token::LineTerminator,
            Token::LineTerminator,
            Token::KAwait,
            Token::IdentifierName(internship::IStr::new("checkEnvironmentVariables")),
            Token::LRound,
            Token::IdentifierName(internship::IStr::new("params")),
            Token::RRound,
            Token::Semicolon,
            Token::LineTerminator,
            Token::KAwait,
            Token::IdentifierName(internship::IStr::new("validateVersion")),
            Token::LRound,
            Token::IdentifierName(internship::IStr::new("params")),
            Token::RRound,
            Token::Semicolon,
            Token::LineTerminator,
            Token::KAwait,
            Token::IdentifierName(internship::IStr::new("checkUncommittedChanges")),
            Token::LRound,
            Token::IdentifierName(internship::IStr::new("params")),
            Token::RRound,
            Token::Semicolon,
            Token::LineTerminator,
            Token::KAwait,
            Token::IdentifierName(internship::IStr::new("checkNpmPermissions")),
            Token::LRound,
            Token::IdentifierName(internship::IStr::new("params")),
            Token::RRound,
            Token::Semicolon,
            Token::LineTerminator,
            Token::KAwait,
            Token::IdentifierName(internship::IStr::new("updateGit")),
            Token::LRound,
            Token::IdentifierName(internship::IStr::new("params")),
            Token::RRound,
            Token::Semicolon,
            Token::LineTerminator,
            Token::KAwait,
            Token::IdentifierName(internship::IStr::new("checkCircleCiStatus")),
            Token::LRound,
            Token::IdentifierName(internship::IStr::new("params")),
            Token::RRound,
            Token::Semicolon,
            Token::LineTerminator,
            Token::KAwait,
            Token::IdentifierName(internship::IStr::new("installYarnDependencies")),
            Token::LRound,
            Token::IdentifierName(internship::IStr::new("params")),
            Token::RRound,
            Token::Semicolon,
            Token::LineTerminator,
            Token::KAwait,
            Token::IdentifierName(internship::IStr::new("checkPackageDependencies")),
            Token::LRound,
            Token::IdentifierName(internship::IStr::new("params")),
            Token::RRound,
            Token::Semicolon,
            Token::LineTerminator,
            Token::KAwait,
            Token::IdentifierName(internship::IStr::new("updateYarnDependencies")),
            Token::LRound,
            Token::IdentifierName(internship::IStr::new("params")),
            Token::RRound,
            Token::Semicolon,
            Token::LineTerminator,
            Token::KAwait,
            Token::IdentifierName(internship::IStr::new("runAutomatedTests")),
            Token::LRound,
            Token::IdentifierName(internship::IStr::new("params")),
            Token::RRound,
            Token::Semicolon,
            Token::LineTerminator,
            Token::LineTerminator,
            Token::LineTerminator,
            Token::LineTerminator,
            Token::KAwait,
            Token::IdentifierName(internship::IStr::new("updatePackageVersions")),
            Token::LRound,
            Token::LCurly,
            Token::LineTerminator,
            Token::TripleDot,
            Token::IdentifierName(internship::IStr::new("params")),
            Token::Comma,
            Token::LineTerminator,
            Token::IdentifierName(internship::IStr::new("packages")),
            Token::Colon,
            Token::IdentifierName(internship::IStr::new("getPackages")),
            Token::LRound,
            Token::RRound,
            Token::Comma,
            Token::LineTerminator,
            Token::RCurly,
            Token::RRound,
            Token::Semicolon,
            Token::LineTerminator,
            Token::KAwait,
            Token::IdentifierName(internship::IStr::new("updateNoopRendererDependencies")),
            Token::LRound,
            Token::IdentifierName(internship::IStr::new("params")),
            Token::RRound,
            Token::Semicolon,
            Token::LineTerminator,
            Token::KAwait,
            Token::IdentifierName(internship::IStr::new("buildArtifacts")),
            Token::LRound,
            Token::IdentifierName(internship::IStr::new("params")),
            Token::RRound,
            Token::Semicolon,
            Token::LineTerminator,
            Token::KAwait,
            Token::IdentifierName(internship::IStr::new("runAutomatedBundleTests")),
            Token::LRound,
            Token::IdentifierName(internship::IStr::new("params")),
            Token::RRound,
            Token::Semicolon,
            Token::LineTerminator,
            Token::KAwait,
            Token::IdentifierName(internship::IStr::new("addGitTag")),
            Token::LRound,
            Token::IdentifierName(internship::IStr::new("params")),
            Token::RRound,
            Token::Semicolon,
            Token::LineTerminator,
            Token::KAwait,
            Token::IdentifierName(internship::IStr::new("printPostBuildSummary")),
            Token::LRound,
            Token::IdentifierName(internship::IStr::new("params")),
            Token::RRound,
            Token::Semicolon,
            Token::LineTerminator,
            Token::RCurly,
            Token::KCatch,
            Token::LRound,
            Token::IdentifierName(internship::IStr::new("error")),
            Token::RRound,
            Token::LCurly,
            Token::LineTerminator,
            Token::IdentifierName(internship::IStr::new("logUpdate")),
            Token::Dot,
            Token::IdentifierName(internship::IStr::new("clear")),
            Token::LRound,
            Token::RRound,
            Token::Semicolon,
            Token::LineTerminator,
            Token::LineTerminator,
            Token::KConst,
            Token::IdentifierName(internship::IStr::new("message")),
            Token::Assign,
            Token::IdentifierName(internship::IStr::new("error")),
            Token::Dot,
            Token::IdentifierName(internship::IStr::new("message")),
            Token::Dot,
            Token::IdentifierName(internship::IStr::new("trim")),
            Token::LRound,
            Token::RRound,
            Token::Dot,
            Token::IdentifierName(internship::IStr::new("replace")),
            Token::LRound,
            Token::StringLiteral(String::from("\n")),
            Token::RRound,
            Token::Semicolon,
            Token::LineTerminator,
            Token::KConst,
            Token::IdentifierName(internship::IStr::new("stack")),
            Token::Assign,
            Token::IdentifierName(internship::IStr::new("error")),
            Token::Dot,
            Token::IdentifierName(internship::IStr::new("stack")),
            Token::Dot,
            Token::IdentifierName(internship::IStr::new("replace")),
            Token::LRound,
            Token::IdentifierName(internship::IStr::new("error")),
            Token::Dot,
            Token::IdentifierName(internship::IStr::new("message")),
            Token::Comma,
            Token::StringLiteral(String::new()),
            Token::RRound,
            Token::Semicolon,
            Token::LineTerminator,
            Token::LineTerminator,
            Token::IdentifierName(internship::IStr::new("console")),
            Token::Dot,
            Token::IdentifierName(internship::IStr::new("log")),
            Token::LRound,
            Token::LineTerminator,
            Token::StringLiteral(String::from("chalk.bgRed.white(\' ERROR \')")),
            Token::LineTerminator,
            Token::RRound,
            Token::Semicolon,
            Token::LineTerminator,
            Token::LineTerminator,
            Token::IdentifierName(internship::IStr::new("process")),
            Token::Dot,
            Token::IdentifierName(internship::IStr::new("exit")),
            Token::LRound,
            Token::NumericLiteral(Number::new(1, 0, 1, 10)),
            Token::RRound,
            Token::Semicolon,
            Token::LineTerminator,
            Token::RCurly,
            Token::LineTerminator,
            Token::RCurly,
            Token::Semicolon,
            Token::LineTerminator,
            Token::LineTerminator,
            Token::LineTerminator,
            Token::LineTerminator,
            Token::IdentifierName(internship::IStr::new("exec")),
            Token::LRound,
            Token::StringLiteral(String::from("yarn install")),
            Token::Comma,
            Token::LCurly,
            Token::IdentifierName(internship::IStr::new("cwd")),
            Token::Colon,
            Token::IdentifierName(internship::IStr::new("__dirname")),
            Token::RCurly,
            Token::Comma,
            Token::LRound,
            Token::IdentifierName(internship::IStr::new("error")),
            Token::Comma,
            Token::IdentifierName(internship::IStr::new("stdout")),
            Token::Comma,
            Token::IdentifierName(internship::IStr::new("stderr")),
            Token::RRound,
            Token::AssignBigger,
            Token::LCurly,
            Token::LineTerminator,
            Token::KIf,
            Token::LRound,
            Token::IdentifierName(internship::IStr::new("error")),
            Token::RRound,
            Token::LCurly,
            Token::LineTerminator,
            Token::IdentifierName(internship::IStr::new("console")),
            Token::Dot,
            Token::IdentifierName(internship::IStr::new("error")),
            Token::LRound,
            Token::IdentifierName(internship::IStr::new("error")),
            Token::RRound,
            Token::Semicolon,
            Token::LineTerminator,
            Token::IdentifierName(internship::IStr::new("process")),
            Token::Dot,
            Token::IdentifierName(internship::IStr::new("exit")),
            Token::LRound,
            Token::NumericLiteral(Number::new(1, 0, 1, 10)),
            Token::RRound,
            Token::Semicolon,
            Token::LineTerminator,
            Token::RCurly,
            Token::KElse,
            Token::LCurly,
            Token::LineTerminator,
            Token::IdentifierName(internship::IStr::new("run")),
            Token::LRound,
            Token::RRound,
            Token::Semicolon,
            Token::LineTerminator,
            Token::RCurly,
            Token::LineTerminator,
            Token::RCurly,
            Token::RRound,
            Token::Semicolon,
            Token::LineTerminator,
            Token::LineTerminator,
            Token::EOF
        ]
    );
}
