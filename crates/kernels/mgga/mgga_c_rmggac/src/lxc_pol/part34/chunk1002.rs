//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1002/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1002<F: Float>(t75231: F, t75235: F, t3219: F, t9087: F, t14639: F, t2412: F, t75238: F, t75241: F, t1614: F, t3204: F, t15489: F, t16043: F) -> (F, F, F, F, F, F, F, F) {
    let t77514 = F::new(0.2553875993597870364e-4) * t75231;
    let t77515 = F::new(0.1702583995731913576e-4) * t75235;
    let t77516 = t9087 * t3219;
    let t77517 = F::new(0.42564599893297839398e-5) * t77516;
    let t77518 = t2412 * t14639;
    let t77519 = F::new(0.42564599893297839398e-5) * t77518;
    let t77520 = F::new(0.16263363996404810741e-4) * t75238;
    let t77521 = F::new(0.16263363996404810741e-4) * t75241;
    let t77525 = t3204 * t1614;
    let t77528 = t16043 * t15489;
    (t77514, t77515, t77517, t77519, t77520, t77521, t77525, t77528)
}
