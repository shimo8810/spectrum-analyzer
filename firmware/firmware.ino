// PINOUT
const uint8_t DATA_PIN = 10;
const uint8_t LATCH_PIN = 9;
const uint8_t CLOCK_PIN = 8;
const uint8_t ROUT_PIN = A1;
const uint8_t LOUT_PIN = A2;
const uint8_t STROBE_PIN = 3;
const uint8_t RESET_PIN = 7;

// LED Matrix constants
const uint8_t NUM_LEVEL = 10;
const uint8_t NUM_BAND = 7;
const uint8_t NUM_BARS = NUM_BAND * 2;

// LED Matrix data
uint32_t MATRIX_DATA[NUM_BARS];

// LED Matrix max level
uint8_t MAX_LEVEL[NUM_BARS];
uint8_t MAX_COUNT[NUM_BARS];
uint8_t MAX_DOWN[NUM_BARS];

/**
 * @brief update led matrix using shift register
 *
 * @param data_pin
 * @param latch_pin
 * @param clock_pin
 * @param data
 */
void updateLedMatrix(const uint8_t data_pin, const uint8_t latch_pin,
                     const uint8_t clock_pin, const uint32_t &data) {
  digitalWrite(latch_pin, LOW);
  for (int i = 0; i < (NUM_LEVEL + NUM_BARS); ++i) {
    digitalWrite(data_pin, (data >> i) & 1);
    digitalWrite(clock_pin, HIGH);
    digitalWrite(clock_pin, LOW);
  }
  digitalWrite(latch_pin, HIGH);
}

/**
 * @brief convert data to led matrix format
 *
 * @param data
 * @return uint32_t
 */
uint32_t convertData(const uint16_t data) {
  // LED bar bits position
  static const uint32_t bar_pos[] = {
      0x20, 0x10, 0x400, 0x1000, 0x4000,  0x80000,  0x400000,
      0x40, 0x80, 0x800, 0x2000, 0x40000, 0x100000, 0x200000};

  // LED bits position
  static const uint32_t led_pos[] = {
      0x1, 0x2, 0x4, 0x8, 0x8000, 0x800000, 0x20000, 0x10000, 0x100, 0x200,
  };

  uint32_t bits = bar_pos[data >> NUM_LEVEL];

  for (uint32_t i = 0; i < NUM_LEVEL; ++i) {
    if ((data >> i) & 1) {
      bits += led_pos[i];
    }
  }
  return bits;
}

/**
 * @brief create led matrix data
 *
 * @param level
 * @param bar
 * @return uint32_t
 */
uint32_t createData(const uint8_t level, const uint8_t bar) {
  uint16_t data = (bar << NUM_LEVEL) + (0x03FF >> (NUM_LEVEL - level));

  if (MAX_LEVEL[bar] <= level) {
    // 最大を更新してカウント値と落下状態を更新
    MAX_LEVEL[bar] = level;
    MAX_DOWN[bar] = 0;
    MAX_COUNT[bar] = 0;
  } else if (MAX_LEVEL[bar] && ++MAX_COUNT[bar] >= 10) {
    if (MAX_LEVEL[bar] >= 5) {
      MAX_DOWN[bar] = 1;
    }
    MAX_LEVEL[bar] -= 1;
    MAX_COUNT[bar] = 10;
  }
  if (MAX_LEVEL[bar] >= 5 || (MAX_LEVEL[bar] && MAX_DOWN[bar])) {
    data |= 1 << (MAX_LEVEL[bar] - 1);
  }

  return convertData(data);
}

/**
 * @brief logscale function
 *
 * @param value
 * @return uint8_t
 */
uint8_t logscale(const int &value) {
  static const int lut[] = {141, 168, 201, 243, 296, 362, 446, 552, 685, 852};
  // static const int lut[] = {65, 85, 112, 148, 195, 257, 338, 446, 589,
  // 777};
  uint8_t level = 0;
  for (level = 0; level < NUM_LEVEL; ++level) {
    if (value < lut[level]) {
      break;
    }
  }
  return level;
}

/**
 * @brief setup function
 *
 */
void setup() {
  // setup led matrix
  pinMode(DATA_PIN, OUTPUT);
  pinMode(LATCH_PIN, OUTPUT);
  pinMode(CLOCK_PIN, OUTPUT);

  // clear LED Matrix
  updateLedMatrix(DATA_PIN, LATCH_PIN, CLOCK_PIN, 0);
  delay(100);

  // setup msgeq7
  pinMode(RESET_PIN, OUTPUT);
  pinMode(STROBE_PIN, OUTPUT);

  // init msgeq7
  digitalWrite(STROBE_PIN, LOW);
  delay(1);
  digitalWrite(RESET_PIN, HIGH);
  delay(1);
  digitalWrite(STROBE_PIN, HIGH);
  delay(1);
  digitalWrite(STROBE_PIN, LOW);
  delay(1);
  digitalWrite(RESET_PIN, LOW);
  delay(5);
}

/**
 * @brief loop function
 *
 */
void loop() {
  static uint8_t band = 0;

  // linear scale
  // uint8_t right_level = (uint8_t)(analogRead(ROUT_PIN) / 94);
  // uint8_t left_level = (uint8_t)(analogRead(LOUT_PIN) / 94);

  // read analog data
  uint8_t right_level = logscale(analogRead(ROUT_PIN));
  uint8_t left_level = logscale(analogRead(LOUT_PIN));

  digitalWrite(STROBE_PIN, HIGH);
  delayMicroseconds(30);
  digitalWrite(STROBE_PIN, LOW);

  // update led matrix
  MATRIX_DATA[band] = createData(right_level, band);
  MATRIX_DATA[band + NUM_BAND] = createData(left_level, band + NUM_BAND);

  for (uint8_t i = 0; i < NUM_BARS; ++i) {
    updateLedMatrix(DATA_PIN, LATCH_PIN, CLOCK_PIN, MATRIX_DATA[i]);
    delayMicroseconds(10);
  }

  // update band
  band += 1;
  if (band >= NUM_BAND) {
    band = 0;
  }
}
