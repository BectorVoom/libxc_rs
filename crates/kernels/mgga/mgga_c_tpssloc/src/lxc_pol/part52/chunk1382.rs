//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1382/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1382<F: Float>(t26142: F, t7266: F, t25985: F, t8690: F, t120064: F, t120067: F, t120069: F, t120072: F, t120075: F, t120078: F, t120079: F, t120083: F, t120085: F, t120086: F, t120088: F) -> F {
    let t123168 = t7266 * t26142;
    let t123173 = t8690 * t25985;
    let t123175 = -F::new(2.0) * t120064 - t120067 - t120069 + t120072 - t120075 + t120078 - F::new(2.0) * t123168 + F::new(3.0) * t120079 - t120083 + t120085 + F::new(3.0) * t120086 + F::new(3.0) * t120088 + F::new(3.0) * t123173;
    t123175
}
