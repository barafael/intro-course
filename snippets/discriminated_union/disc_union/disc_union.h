#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * An enum representing JSON data.
 * For a production-quality version of this, see
 * [`serde_json::Value`].
 */
// marker-start:disc_union_tag_enum
typedef enum Value_Tag {
  Null,
  Bool,
  Number,
  String,
} Value_Tag;
// marker-end:disc_union_tag_enum

typedef struct Value {
  Value_Tag tag;
  union {
    struct {
      bool bool_;
    };
    struct {
      double number;
    };
    struct {
      const char *string;
    };
  };
} Value;

struct Value create_number(double number);

const char *format_value(const struct Value *value);

extern int puts(const char *s);

void puts_value(const struct Value *value);
