# frozen_string_literal: true

require "bundler/gem_tasks"
require "rake/testtask"

Rake::TestTask.new(:test) do |t|
  t.libs << "test"
  t.libs << "lib"
  t.test_files = FileList["test/**/test_*.rb"]
end

require "standard/rake"

require "rb_sys/extensiontask"

task build: :compile

RbSys::ExtensionTask.new("taffyrb") do |ext|
  ext.lib_dir = "lib/taffyrb"
end

task default: %i[compile test standard]
