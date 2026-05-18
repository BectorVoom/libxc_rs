//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 1215/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk1215<F: Float>(t25: F, t1788: F, t2225: F, t2221: F, t2223: F, t12130: F, t11987: F, t1408: F, t2: F, t3704: F, t1298: F, t15941: F, t16: F, t2249: F, t3665: F, t5170: F, t5173: F, t584: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t26 = t25 <= zeta_threshold;
    let t15982 = t2225 * t1788;
    let t15983 = F::new(20.0) * t15982;
    let t15984 = t2221 * t1788;
    let t15985 = F::new(12.0) * t15984;
    let t15986 = t2223 * t1788;
    let t15987 = F::new(32.0) * t15986;
    let t15988 = F::new(2.0) * t12130;
    let t15989 = t11987 * t1408;
    let t15992 = t3704 * t2;
    let t16002 = piecewise3::<f64>(t26, F::new(0.0), F::new(8.0) / F::new(27.0) * t15989 * t3665 - F::new(8.0) / F::new(9.0) * t15992 * t15941 - F::new(2.0) / F::new(9.0) * t5170 * t2249 + F::new(4.0) / F::new(3.0) * t1298 * t584 - F::new(4.0) * t5173 * t16);
    (t15983, t15985, t15987, t15988, t16002)
}
