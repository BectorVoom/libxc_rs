//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 919/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk919<F: Float>(t15872: F, t4965: F, t69954: F, t71551: F, t71552: F, t71565: F, t75678: F, t77733: F, t77737: F, t77741: F, t77745: F, t77750: F, t77755: F, t77760: F, t77765: F, t77770: F, t77772: F, t77773: F) -> (F,) {
    let t80288 = t71551 - t71552 - t77733 + t77737 - t77741 - t77745 - t77750 + t77755 - t77760 - t77765 + t77770 + 0.39914139006212695214e-1 * t4965 * t15872 - t69954 + t77772 - t71565 - t75678 + t77773;
    (t80288,)
}
