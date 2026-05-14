//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1056/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1056<F: Float>(t12739: F, t12740: F, t12741: F, t12742: F, t12745: F, t12747: F, t12748: F, t12750: F, t7954: F, t7960: F, t7972: F, t7975: F, t9886: F, t9903: F, t9906: F, t9954: F) -> (F,) {
    let t12904 = t9886 - t12739 + t12740 + t12741 + t9903 - t9906 - t7954 - t12742 + t12745 - t7960 + t7972 + t7975 + t12747 - t12748 + t12750 - t9954;
    (t12904,)
}
