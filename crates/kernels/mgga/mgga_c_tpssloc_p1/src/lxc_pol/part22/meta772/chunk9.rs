//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2641/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2641<F: Float>(t22334: F, t225: F, t1238: F, t1251: F, t1252: F, t15797: F, t15820: F, t1761: F, t19209: F, t19214: F, t19220: F, t19232: F, t19234: F, t22007: F, t22008: F, t3593: F, t3598: F, t45350: F, t4945: F, t5055: F, t5060: F, t5088: F, t5089: F, t6244: F, t6267: F, t6268: F, t66822: F) -> F {
    let t73856 = t22334 * t225;
    let t73885 = F::new(24.0) * t1238 * t1251 * t22007 * t45350 + F::new(6.0) * t1238 * t3598 * t5088 * t6267 - F::new(3.0) * t1252 * t73856 - F::new(3.0) * t15797 * t6268 + F::new(6.0) * t15820 * t6244 - F::new(3.0) * t15820 * t6268 - F::new(6.0) * t1761 * t66822 - F::new(3.0) * t19209 * t5055 + F::new(12.0) * t19214 * t4945 + F::new(6.0) * t19220 * t4945 - F::new(3.0) * t19232 * t5089 + F::new(12.0) * t19234 * t5060 - F::new(6.0) * t22008 * t3593;
    t73885
}
