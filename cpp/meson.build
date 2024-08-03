project('cpp_challenges', 'cpp',
  version : '0.1',
  default_options : ['warning_level=3'])

ut_dep = dependency('boost-ext-ut')

exe = executable('app', 'main.cc', dependencies: ut_dep)

test('basic', exe)
