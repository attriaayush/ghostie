class Ghostie < Formula
  desc "Github notifications in your terminal"
  version "v0.1.0"
  homepage "https://github.com/attriaayush/ghostie"
  license "MIT"

  depends_on "sqlite"

  on_macos do
    url "https://github.com/attriaayush/ghostie/releases/download/#{version}/ghostie-#{version}-x86_64-apple-darwin.tar.gz"
    sha256 "eb1fcc3eafb4588dc0c62fda6134b808bccca57e428dd6d1194ddfce05a6e5d7"
  end

  on_linux do
    url "https://github.com/attriaayush/ghostie/releases/download/#{version}/ghostie-#{version}-x86_64-unknown-linux-gnu.tar.gz"
    sha256 "d2bfea37c323d492406f749a7a3f7f803fa05a4ed603caab79bab3f97855a667"
  end

  def install
    bin.install "ghostie"
  end

   def caveats
    <<~EOS
      ONE MORE STEP!
      Add the following to the end of your ~/.bashrc, ~/.zshrc, or ~/.config/fish/config.fish file.
    EOS
   end
end
