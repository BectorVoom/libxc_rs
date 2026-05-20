//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2397/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2397<F: Float>(t13623: F, t5705: F, t17271: F, t4378: F, t21180: F, t2798: F, t896: F, t2815: F, t136: F, t68569: F, t908: F, t41684: F, t48946: F, t48947: F, t48956: F, t59657: F, t68442: F, t68444: F, t68446: F, t68448: F, t68479: F, t68483: F, t68486: F, t68489: F, t68492: F, t68494: F, t68498: F, t68571: F, t68577: F, t68580: F, t68583: F) -> (F, F, F, F, F, F) {
    let t68638 = t13623 * t5705;
    let t68640 = t4378 * t17271;
    let t68643 = t2798 * t21180 * t896;
    let t68646 = t2815 * t21180 * t896;
    let t68649 = t136 * t908 * t68569;
    let t68673 = F::new(2.0) / F::new(3.0) * t68442 + t68444 / F::new(9.0) + F::new(10.0) / F::new(81.0) * t68446 - F::new(4.0) / F::new(9.0) * t68448 + t48946 - t48947 - t48956 + F::new(28.0) / F::new(81.0) * t41684 - F::new(80.0) / F::new(81.0) * t68479 - F::new(8.0) * t68483 + F::new(4.0) * t68486 - F::new(2.0) / F::new(3.0) * t68489 - F::new(2.0) / F::new(3.0) * t68492 + F::new(2.0) / F::new(9.0) * t68494 - F::new(2.0) / F::new(3.0) * t68498 - F::new(8.0) / F::new(27.0) * t59657 - t68571 / F::new(3.0) + F::new(8.0) * t68577 - F::new(6.0) * t68580 + F::new(2.0) * t68583;
    (t68638, t68640, t68643, t68646, t68649, t68673)
}
