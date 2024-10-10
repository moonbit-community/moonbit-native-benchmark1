package com.moonbitlang.showcase.fft;

import static org.junit.jupiter.api.Assertions.assertArrayEquals;

import java.util.Arrays;
import java.util.function.Function;
import java.util.stream.IntStream;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.CsvSource;

// Ported from
// <https://github.com/dntj/jsfft/blob/19d65ed77f5590be1cab91186e42ca7047dd09db/test/test-fft.js>.
class CooleyTukeyTest {
  static void assertFFTEquals(Complex[] expected, Complex[] inputs) {
    var outputs = inputs.clone();
    assertFFTEqualsImpl(expected, outputs);
  }

  static void assertFFTEquals(Complex[] expected, double[] reals) {
    var outputs = new Complex[reals.length];
    for (var i = 0; i < reals.length; i++) {
      outputs[i] = new Complex(reals[i], 0);
    }
    assertFFTEqualsImpl(expected, outputs);
  }

  private static void assertFFTEqualsImpl(Complex[] expected, Complex[] inputs) {
    CooleyTukey.fft(inputs);
    assertArrayEquals(expected, inputs);
  }

  @ParameterizedTest(name = "{index} => calculates the real FFT of {0}: {1}")
  @CsvSource(
      delimiter = '|',
      textBlock =
          """
                    constant | [1, 1, 1, 1]   | [2, 0, 0, 0]
                       delta | [1, 0, 0, 0]   | [.5, .5, .5, .5]
              single hi freq | [1, -1, 1, -1] | [0, 0, 2, 0]
              single lo freq | [1, 0, -1, -0] | [0, 1, 0, 1]
                hi freq + dc | [1, 0, 1, 0]   | [1, 0, 1, 0]
          """)
  void testFFTReals(String _testName, String realsStr, String expectedStr) {
    Function<String, double[]> parseDoubles =
        str ->
            Arrays.asList(str.replace("[", "").replace("]", "").split(",")).stream()
                .mapToDouble(c -> Double.parseDouble(c.toString().trim()))
                .toArray();
    var reals = parseDoubles.apply(realsStr);

    var expectedDoubles = parseDoubles.apply(expectedStr);
    var expected =
        Arrays.stream(expectedDoubles).mapToObj(d -> new Complex(d, 0)).toArray(Complex[]::new);

    assertFFTEquals(expected, reals);
  }

  @ParameterizedTest(name = "{index} => calculates the complex FFT of {0}: {1}")
  @CsvSource(
      delimiter = '|',
      // https://introcs.cs.princeton.edu/java/97data/FFT.java.html
      textBlock =
          """
- | [-.03480425839330703, .07910192950176387, .7233322451735928, .1659819820667019] | [.9336118983487516, -.7581365035668999, .44344407521182005, -.7581365035668999] | [0, .08688005256493803, 0, -.08688005256493803]
""")
  void testFFTReals(
      String _testName, String realsStr, String expectedRealsStr, String expectedImagsStr) {
    Function<String, double[]> parseDoubles =
        str ->
            Arrays.asList(str.replace("[", "").replace("]", "").split(",")).stream()
                .mapToDouble(c -> Double.parseDouble(c.toString().trim()))
                .toArray();
    var reals = parseDoubles.apply(realsStr);

    var expectedReals = parseDoubles.apply(expectedRealsStr);
    var expectedImags = parseDoubles.apply(expectedImagsStr);
    final var n = expectedReals.length;
    final var factor = 1 / Math.sqrt(n);
    var expected =
        IntStream.range(0, n)
            .mapToObj(i -> new Complex(expectedReals[i] * factor, expectedImags[i] * factor))
            .toArray(Complex[]::new);

    assertFFTEquals(expected, reals);
  }
}
