//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1483/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1483<F: Float>(t1420: F, t1423: F, t19368: F, t19390: F, t20217: F, t20246: F, t20255: F, t20258: F, t20261: F, t2267: F, t2274: F, t39: F, t39159: F, t39168: F, t39210: F, t3981: F, t3990: F, t43: F, t51: F, t5398: F, t5416: F, t5421: F, t5424: F, t55: F, t56: F, t75836: F, t75847: F, t75912: F, t78505: F) -> F {
    let t79692 = F::cast_from(5.0_f64) / F::cast_from(162.0_f64) * t39 * t39159 * t75836 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t39 * t43 * t75912 + F::cast_from(20944.0_f64) / F::cast_from(81.0_f64) * t78505 * t56 + F::cast_from(12320.0_f64) / F::cast_from(81.0_f64) * t20246 * t1423 - F::cast_from(440.0_f64) / F::cast_from(9.0_f64) * t5416 * t5424 + F::cast_from(440.0_f64) / F::cast_from(27.0_f64) * t5416 * t5421 - F::cast_from(40.0_f64) / F::cast_from(81.0_f64) * t1420 * t20255 + F::cast_from(80.0_f64) / F::cast_from(9.0_f64) * t1420 * t20261 + F::cast_from(5.0_f64) / F::cast_from(162.0_f64) * t51 * t39168 * t75836 - F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t51 * t55 * t75912 - F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t39 * t19368 * t5398 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t39 * t2267 * t75847 + F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t39 * t3981 * t20217 - F::cast_from(80.0_f64) / F::cast_from(9.0_f64) * t1420 * t20258 + F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t51 * t19390 * t5398 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t51 * t2274 * t75847 + F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t51 * t3990 * t20217 - t39210;
    t79692
}
