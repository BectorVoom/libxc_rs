//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1990/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1990<F: Float>(t11652: F, t11665: F, t11678: F, t11692: F, t11699: F, t11703: F, t1174: F, t1218: F, t1232: F, t15560: F, t15564: F, t15569: F, t15574: F, t15580: F, t15581: F, t15584: F, t15587: F, t15591: F, t15594: F, t3496: F, t3580: F, t4950: F, t5002: F) -> F {
    let t15601 = -t11678 * t15560 / F::new(2304.0) + t11692 * t15564 / F::new(4608.0) + t15569 * t3580 / F::new(432.0) - t15574 - t11665 * t4950 / F::new(2304.0) - t11652 / F::new(4608.0) - t15580 - t1174 * t15581 / F::new(72.0) - t1174 * t15584 / F::new(144.0) - t1174 * t15587 / F::new(48.0) + t15591 * t1218 / F::new(1536.0) - t15594 * t1232 / F::new(2304.0) + t5002 * t3496 / F::new(3072.0) - t11699 / F::new(3456.0) + t11703 / F::new(4608.0);
    t15601
}
