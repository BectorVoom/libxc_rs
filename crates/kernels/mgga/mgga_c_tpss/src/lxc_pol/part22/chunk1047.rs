//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1047/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1047<F: Float>(t9883: F, t9887: F, t9890: F, t9900: F, t9913: F, t123: F, t1613: F, t2349: F, t1170: F, t4432: F, t9916: F, t1614: F, t3305: F, t9957: F, t7945: F, t7954: F, t7960: F, t7972: F, t7975: F, t9886: F, t9903: F, t9906: F, t9954: F, t9956: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t12738 = 16.0 * t9883;
    let t12739 = 0.34631718211362927518e2 * t9887;
    let t12740 = 0.23392894490538584828e1 * t9890;
    let t12741 = 2.0 * t9900;
    let t12742 = 32.0 * t9913;
    let t12743 = t1613 * t123;
    let t12744 = t12743 * t2349;
    let t12745 = 0.10843581300301739842e-1 * t12744;
    let t12746 = t1170 * t4432;
    let t12747 = 8.0 * t12746;
    let t12748 = 0.5848223622634646207e0 * t9916;
    let t12749 = t3305 * t1614;
    let t12750 = 20.0 * t12749;
    let t12751 = 0.4883052614935078681e-3 * t9957;
    let t12752 = t7945 - t12738 + t9886 - t12739 + t12740 + t12741 + t9903 - t9906 - t7954 - t12742 + t12745 - t7960 + t7972 + t7975 + t12747 - t12748 + t12750 - t9954 + t9956 + t12751;
    (t12738, t12739, t12740, t12741, t12742, t12745, t12747, t12748, t12750, t12751, t12752)
}
