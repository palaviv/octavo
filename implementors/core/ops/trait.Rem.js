(function() {var implementors = {};
implementors['libc'] = [];implementors['typenum'] = ["impl&lt;Ur: <a class='trait' href='typenum/marker_traits/trait.Unsigned.html' title='typenum::marker_traits::Unsigned'>Unsigned</a>, Br: <a class='trait' href='typenum/marker_traits/trait.Bit.html' title='typenum::marker_traits::Bit'>Bit</a>&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/ops/trait.Rem.html' title='core::ops::Rem'>Rem</a>&lt;<a class='struct' href='typenum/uint/struct.UInt.html' title='typenum::uint::UInt'>UInt</a>&lt;Ur, Br&gt;&gt; for <a class='enum' href='typenum/uint/enum.UTerm.html' title='typenum::uint::UTerm'>UTerm</a>","impl&lt;Ul: <a class='trait' href='typenum/marker_traits/trait.Unsigned.html' title='typenum::marker_traits::Unsigned'>Unsigned</a>, Bl: <a class='trait' href='typenum/marker_traits/trait.Bit.html' title='typenum::marker_traits::Bit'>Bit</a>, Ur: <a class='trait' href='typenum/marker_traits/trait.Unsigned.html' title='typenum::marker_traits::Unsigned'>Unsigned</a>, Br: <a class='trait' href='typenum/marker_traits/trait.Bit.html' title='typenum::marker_traits::Bit'>Bit</a>&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/ops/trait.Rem.html' title='core::ops::Rem'>Rem</a>&lt;<a class='struct' href='typenum/uint/struct.UInt.html' title='typenum::uint::UInt'>UInt</a>&lt;Ur, Br&gt;&gt; for <a class='struct' href='typenum/uint/struct.UInt.html' title='typenum::uint::UInt'>UInt</a>&lt;Ul, Bl&gt; <span class='where'>where <a class='struct' href='typenum/uint/struct.UInt.html' title='typenum::uint::UInt'>UInt</a>&lt;Ul, Bl&gt;: <a class='trait' href='typenum/type_operators/trait.Cmp.html' title='typenum::type_operators::Cmp'>Cmp</a>&lt;<a class='struct' href='typenum/uint/struct.UInt.html' title='typenum::uint::UInt'>UInt</a>&lt;Ur, Br&gt;&gt;, <a class='struct' href='typenum/uint/struct.UInt.html' title='typenum::uint::UInt'>UInt</a>&lt;Ul, Bl&gt;: PrivateDivFirstStep&lt;<a class='struct' href='typenum/uint/struct.UInt.html' title='typenum::uint::UInt'>UInt</a>&lt;Ul, Bl&gt;::Output, <a class='struct' href='typenum/uint/struct.UInt.html' title='typenum::uint::UInt'>UInt</a>&lt;Ur, Br&gt;&gt;</span>","impl&lt;I: <a class='trait' href='typenum/marker_traits/trait.Integer.html' title='typenum::marker_traits::Integer'>Integer</a> + <a class='trait' href='typenum/marker_traits/trait.NonZero.html' title='typenum::marker_traits::NonZero'>NonZero</a>&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/ops/trait.Rem.html' title='core::ops::Rem'>Rem</a>&lt;I&gt; for <a class='enum' href='typenum/int/enum.Z0.html' title='typenum::int::Z0'>Z0</a>","impl&lt;Ul: <a class='trait' href='typenum/marker_traits/trait.Unsigned.html' title='typenum::marker_traits::Unsigned'>Unsigned</a> + <a class='trait' href='typenum/marker_traits/trait.NonZero.html' title='typenum::marker_traits::NonZero'>NonZero</a>, Ur: <a class='trait' href='typenum/marker_traits/trait.Unsigned.html' title='typenum::marker_traits::Unsigned'>Unsigned</a> + <a class='trait' href='typenum/marker_traits/trait.NonZero.html' title='typenum::marker_traits::NonZero'>NonZero</a>&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/ops/trait.Rem.html' title='core::ops::Rem'>Rem</a>&lt;<a class='struct' href='typenum/int/struct.PInt.html' title='typenum::int::PInt'>PInt</a>&lt;Ur&gt;&gt; for <a class='struct' href='typenum/int/struct.PInt.html' title='typenum::int::PInt'>PInt</a>&lt;Ul&gt; <span class='where'>where Ul: <a class='trait' href='https://doc.rust-lang.org/nightly/core/ops/trait.Rem.html' title='core::ops::Rem'>Rem</a>&lt;Ur&gt;, <a class='struct' href='typenum/int/struct.PInt.html' title='typenum::int::PInt'>PInt</a>&lt;Ul&gt;: PrivateRem&lt;Ul::Output, <a class='struct' href='typenum/int/struct.PInt.html' title='typenum::int::PInt'>PInt</a>&lt;Ur&gt;&gt;</span>","impl&lt;Ul: <a class='trait' href='typenum/marker_traits/trait.Unsigned.html' title='typenum::marker_traits::Unsigned'>Unsigned</a> + <a class='trait' href='typenum/marker_traits/trait.NonZero.html' title='typenum::marker_traits::NonZero'>NonZero</a>, Ur: <a class='trait' href='typenum/marker_traits/trait.Unsigned.html' title='typenum::marker_traits::Unsigned'>Unsigned</a> + <a class='trait' href='typenum/marker_traits/trait.NonZero.html' title='typenum::marker_traits::NonZero'>NonZero</a>&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/ops/trait.Rem.html' title='core::ops::Rem'>Rem</a>&lt;<a class='struct' href='typenum/int/struct.NInt.html' title='typenum::int::NInt'>NInt</a>&lt;Ur&gt;&gt; for <a class='struct' href='typenum/int/struct.PInt.html' title='typenum::int::PInt'>PInt</a>&lt;Ul&gt; <span class='where'>where Ul: <a class='trait' href='https://doc.rust-lang.org/nightly/core/ops/trait.Rem.html' title='core::ops::Rem'>Rem</a>&lt;Ur&gt;, <a class='struct' href='typenum/int/struct.PInt.html' title='typenum::int::PInt'>PInt</a>&lt;Ul&gt;: PrivateRem&lt;Ul::Output, <a class='struct' href='typenum/int/struct.NInt.html' title='typenum::int::NInt'>NInt</a>&lt;Ur&gt;&gt;</span>","impl&lt;Ul: <a class='trait' href='typenum/marker_traits/trait.Unsigned.html' title='typenum::marker_traits::Unsigned'>Unsigned</a> + <a class='trait' href='typenum/marker_traits/trait.NonZero.html' title='typenum::marker_traits::NonZero'>NonZero</a>, Ur: <a class='trait' href='typenum/marker_traits/trait.Unsigned.html' title='typenum::marker_traits::Unsigned'>Unsigned</a> + <a class='trait' href='typenum/marker_traits/trait.NonZero.html' title='typenum::marker_traits::NonZero'>NonZero</a>&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/ops/trait.Rem.html' title='core::ops::Rem'>Rem</a>&lt;<a class='struct' href='typenum/int/struct.PInt.html' title='typenum::int::PInt'>PInt</a>&lt;Ur&gt;&gt; for <a class='struct' href='typenum/int/struct.NInt.html' title='typenum::int::NInt'>NInt</a>&lt;Ul&gt; <span class='where'>where Ul: <a class='trait' href='https://doc.rust-lang.org/nightly/core/ops/trait.Rem.html' title='core::ops::Rem'>Rem</a>&lt;Ur&gt;, <a class='struct' href='typenum/int/struct.NInt.html' title='typenum::int::NInt'>NInt</a>&lt;Ul&gt;: PrivateRem&lt;Ul::Output, <a class='struct' href='typenum/int/struct.PInt.html' title='typenum::int::PInt'>PInt</a>&lt;Ur&gt;&gt;</span>","impl&lt;Ul: <a class='trait' href='typenum/marker_traits/trait.Unsigned.html' title='typenum::marker_traits::Unsigned'>Unsigned</a> + <a class='trait' href='typenum/marker_traits/trait.NonZero.html' title='typenum::marker_traits::NonZero'>NonZero</a>, Ur: <a class='trait' href='typenum/marker_traits/trait.Unsigned.html' title='typenum::marker_traits::Unsigned'>Unsigned</a> + <a class='trait' href='typenum/marker_traits/trait.NonZero.html' title='typenum::marker_traits::NonZero'>NonZero</a>&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/ops/trait.Rem.html' title='core::ops::Rem'>Rem</a>&lt;<a class='struct' href='typenum/int/struct.NInt.html' title='typenum::int::NInt'>NInt</a>&lt;Ur&gt;&gt; for <a class='struct' href='typenum/int/struct.NInt.html' title='typenum::int::NInt'>NInt</a>&lt;Ul&gt; <span class='where'>where Ul: <a class='trait' href='https://doc.rust-lang.org/nightly/core/ops/trait.Rem.html' title='core::ops::Rem'>Rem</a>&lt;Ur&gt;, <a class='struct' href='typenum/int/struct.NInt.html' title='typenum::int::NInt'>NInt</a>&lt;Ul&gt;: PrivateRem&lt;Ul::Output, <a class='struct' href='typenum/int/struct.NInt.html' title='typenum::int::NInt'>NInt</a>&lt;Ur&gt;&gt;</span>",];implementors['num_bigint'] = ["impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/ops/trait.Rem.html' title='core::ops::Rem'>Rem</a>&lt;<a class='struct' href='num_bigint/struct.BigUint.html' title='num_bigint::BigUint'>BigUint</a>&gt; for <a class='struct' href='num_bigint/struct.BigUint.html' title='num_bigint::BigUint'>BigUint</a>","impl&lt;'a&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/ops/trait.Rem.html' title='core::ops::Rem'>Rem</a>&lt;&amp;'a <a class='struct' href='num_bigint/struct.BigUint.html' title='num_bigint::BigUint'>BigUint</a>&gt; for <a class='struct' href='num_bigint/struct.BigUint.html' title='num_bigint::BigUint'>BigUint</a>","impl&lt;'a&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/ops/trait.Rem.html' title='core::ops::Rem'>Rem</a>&lt;<a class='struct' href='num_bigint/struct.BigUint.html' title='num_bigint::BigUint'>BigUint</a>&gt; for &amp;'a <a class='struct' href='num_bigint/struct.BigUint.html' title='num_bigint::BigUint'>BigUint</a>","impl&lt;'a, 'b&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/ops/trait.Rem.html' title='core::ops::Rem'>Rem</a>&lt;&amp;'b <a class='struct' href='num_bigint/struct.BigUint.html' title='num_bigint::BigUint'>BigUint</a>&gt; for &amp;'a <a class='struct' href='num_bigint/struct.BigUint.html' title='num_bigint::BigUint'>BigUint</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/ops/trait.Rem.html' title='core::ops::Rem'>Rem</a>&lt;<a class='struct' href='num_bigint/struct.BigInt.html' title='num_bigint::BigInt'>BigInt</a>&gt; for <a class='struct' href='num_bigint/struct.BigInt.html' title='num_bigint::BigInt'>BigInt</a>","impl&lt;'a&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/ops/trait.Rem.html' title='core::ops::Rem'>Rem</a>&lt;&amp;'a <a class='struct' href='num_bigint/struct.BigInt.html' title='num_bigint::BigInt'>BigInt</a>&gt; for <a class='struct' href='num_bigint/struct.BigInt.html' title='num_bigint::BigInt'>BigInt</a>","impl&lt;'a&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/ops/trait.Rem.html' title='core::ops::Rem'>Rem</a>&lt;<a class='struct' href='num_bigint/struct.BigInt.html' title='num_bigint::BigInt'>BigInt</a>&gt; for &amp;'a <a class='struct' href='num_bigint/struct.BigInt.html' title='num_bigint::BigInt'>BigInt</a>","impl&lt;'a, 'b&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/ops/trait.Rem.html' title='core::ops::Rem'>Rem</a>&lt;&amp;'b <a class='struct' href='num_bigint/struct.BigInt.html' title='num_bigint::BigInt'>BigInt</a>&gt; for &amp;'a <a class='struct' href='num_bigint/struct.BigInt.html' title='num_bigint::BigInt'>BigInt</a>",];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()