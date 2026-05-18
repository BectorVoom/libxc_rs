//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1371/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1371<F: Float>(t22574: F, t25988: F, t36740: F, t26168: F, t8607: F, t31747: F, t4028: F, t121159: F, t121160: F, t121162: F, t121165: F, t121169: F, t1849: F, t25965: F, t26977: F, t27147: F, t31246: F, t31532: F, t31722: F, t4077: F, t6517: F, t7042: F, t7472: F, t7941: F) -> F {
    let t121174 = F::new(3.0) * t22574 * t36740 * t25988;
    let t121177 = F::new(3.0) * t8607 * t26168;
    let t121179 = F::new(2.0) * t4028 * t31747;
    let t121180 = t1849 * t31722 - F::new(2.0) * t25965 * t7042 - F::new(2.0) * t26977 * t7472 - F::new(2.0) * t27147 * t6517 + t31246 * t7941 - F::new(2.0) * t31532 * t4077 - t121159 + t121160 - t121162 - t121165 - t121169 - t121174 + t121177 - t121179;
    t121180
}
