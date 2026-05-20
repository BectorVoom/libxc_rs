//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1349/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1349<F: Float>(t13642: F, t13709: F, t17154: F, t17159: F, t17163: F, t17169: F, t17211: F, t17213: F, t17216: F, t17219: F, t17221: F, t17224: F, t17238: F, t17241: F, t17244: F, t17247: F, t17250: F, t17253: F, t17256: F, t17272: F, t17274: F, t17295: F) -> F {
    let t17297 = F::new(0.19419375e1) * t17211 - F::new(0.258925e1) * t17213 - F::new(0.1294625e1) * t17216 - F::cast_from(0.412621875e-1_f64) * t17219 + F::new(0.16504875e0) * t17221 + F::new(0.82524375e-1) * t17224 - F::cast_from(0.33547222222222222222e0_f64) * t17154 + F::new(0.12077e1) * t17159 - F::cast_from(0.40256666666666666666e0_f64) * t17163 - F::new(0.181155e1) * t17169 + t17238 - F::new(0.5519e-1) * t17241 - F::cast_from(0.36793333333333333333e-1_f64) * t17244 - F::new(0.49671e0) * t17247 + F::new(0.33114e0) * t17250 + F::new(0.16557e0) * t17253 - F::new(0.27595e-1) * t17256 + F::new(0.258925e1) * t17272 + F::new(0.16504875e0) * t17274 - F::cast_from(0.18396666666666666667e0_f64) * t13642 + t13709 + t17295;
    t17297
}
