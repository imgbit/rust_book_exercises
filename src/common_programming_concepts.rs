pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

pub fn nth_fibonacci(f: u32) -> u32 {
    if f == 0 {
        0
    } else if f == 1 {
        1
    } else {
        // use recursion
        nth_fibonacci(f - 1) + nth_fibonacci(f - 2)
    }
}

pub fn write_a_christmas_carol() -> String {
    let ordinals = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    let verses = [
        "A partridge in a pear tree.",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    let mut acc = String::from(verses[0]);
    let mut ret = String::new();
    for (index, verse) in verses.iter().enumerate() {
        if index != 0 {
            acc = format!("{},\n{}", verse, acc);
        }
        ret = format!(
            "{}\n{}\n{}\n",
            ret,
            format!(
                "On the {} day of Christmas, my true love gave to me",
                ordinals[index]
            ),
            acc
        );
        if index == 0 {
            acc = format!("And {}", acc.to_lowercase());
        }
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fahrenheit_to_celsius() {
        assert_eq!(fahrenheit_to_celsius(0.0), -17.77777777777778);
        assert_eq!(fahrenheit_to_celsius(68.0), 20.0);
        assert_eq!(fahrenheit_to_celsius(212.0), 100.0);
        assert_eq!(fahrenheit_to_celsius(-1000.0), -573.3333333333334);
    }

    #[test]
    fn test_nth_fibonacci() {
        assert_eq!(nth_fibonacci(0), 0);
        assert_eq!(nth_fibonacci(1), 1);
        assert_eq!(nth_fibonacci(5), 5);
        assert_eq!(nth_fibonacci(10), 55);
    }

    #[test]
    fn test_write_a_christmas_carol() {
        let lyrics = r#"
On the first day of Christmas, my true love gave to me
A partridge in a pear tree.

On the second day of Christmas, my true love gave to me
Two turtle doves,
And a partridge in a pear tree.

On the third day of Christmas, my true love gave to me
Three French hens,
Two turtle doves,
And a partridge in a pear tree.

On the fourth day of Christmas, my true love gave to me
Four calling birds,
Three French hens,
Two turtle doves,
And a partridge in a pear tree.

On the fifth day of Christmas, my true love gave to me
Five golden rings,
Four calling birds,
Three French hens,
Two turtle doves,
And a partridge in a pear tree.

On the sixth day of Christmas, my true love gave to me
Six geese a-laying,
Five golden rings,
Four calling birds,
Three French hens,
Two turtle doves,
And a partridge in a pear tree.

On the seventh day of Christmas, my true love gave to me
Seven swans a-swimming,
Six geese a-laying,
Five golden rings,
Four calling birds,
Three French hens,
Two turtle doves,
And a partridge in a pear tree.

On the eighth day of Christmas, my true love gave to me
Eight maids a-milking,
Seven swans a-swimming,
Six geese a-laying,
Five golden rings,
Four calling birds,
Three French hens,
Two turtle doves,
And a partridge in a pear tree.

On the ninth day of Christmas, my true love gave to me
Nine ladies dancing,
Eight maids a-milking,
Seven swans a-swimming,
Six geese a-laying,
Five golden rings,
Four calling birds,
Three French hens,
Two turtle doves,
And a partridge in a pear tree.

On the tenth day of Christmas, my true love gave to me
Ten lords a-leaping,
Nine ladies dancing,
Eight maids a-milking,
Seven swans a-swimming,
Six geese a-laying,
Five golden rings,
Four calling birds,
Three French hens,
Two turtle doves,
And a partridge in a pear tree.

On the eleventh day of Christmas, my true love gave to me
Eleven pipers piping,
Ten lords a-leaping,
Nine ladies dancing,
Eight maids a-milking,
Seven swans a-swimming,
Six geese a-laying,
Five golden rings,
Four calling birds,
Three French hens,
Two turtle doves,
And a partridge in a pear tree.

On the twelfth day of Christmas, my true love gave to me
Twelve drummers drumming,
Eleven pipers piping,
Ten lords a-leaping,
Nine ladies dancing,
Eight maids a-milking,
Seven swans a-swimming,
Six geese a-laying,
Five golden rings,
Four calling birds,
Three French hens,
Two turtle doves,
And a partridge in a pear tree.
"#;
        assert_eq!(write_a_christmas_carol(), lyrics.to_string());
    }
}
