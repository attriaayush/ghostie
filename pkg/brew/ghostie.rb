class Ghostie < Formula
  version "v0.1.0"
  desc "Ghostie"
  homepage "https://github.com/attriaayush/ghostie/README.md"
  url "https://github.com/attriaayush/ghostie"

  # if OS.mac?
  #     url ""
  #     sha256 ""
  # elsif OS.linux?
  #     url ""
  #     sha256 ""
  # end

  depends_on "sqlite"

  def install
    bin.install = "ghostie"
  end

  def caveats
    <<~EOS
      ONE MORE STEP!
      Add the following to the end of your ~/.bashrc, ~/.zshrc, or ~/.config/fish/config.fish file.
    EOS
end

