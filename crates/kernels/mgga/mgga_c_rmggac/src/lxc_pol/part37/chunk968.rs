//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 968/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk968<F: Float>(t77516: F, t14639: F, t2412: F, t75238: F, t75241: F, t15489: F, t16043: F, t1971: F, t3351: F, t44157: F, t875: F, t44183: F) -> (F, F, F, F, F, F, F) {
    let t77517 = F::new(0.42564599893297839398e-5) * t77516;
    let t77518 = t2412 * t14639;
    let t77519 = F::new(0.42564599893297839398e-5) * t77518;
    let t77520 = F::new(0.16263363996404810741e-4) * t75238;
    let t77521 = F::new(0.16263363996404810741e-4) * t75241;
    let t77528 = t16043 * t15489;
    let t77529 = F::new(0.85129199786595678796e-5) * t77528;
    let t77532 = t3351 * t1971 * t875 * t44157;
    let t77533 = F::new(0.85129199786595678796e-5) * t77532;
    let t77536 = t3351 * t1971 * t875 * t44183;
    (t77517, t77519, t77520, t77521, t77529, t77533, t77536)
}
