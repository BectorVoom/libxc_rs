//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 861/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk861<F: Float>(t69130: F, t2211: F, t8794: F, t118: F, t15547: F, t2085: F, t76143: F, t76145: F, t14512: F, t8568: F, t2344: F, t71876: F, t14516: F, t8526: F, t2329: F, t71882: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t77830 = 0.18183107769496894487e-1 * t69130;
    let t77831 = t2211 * t8794;
    let t77833 = 0.39914139006212695214e-1 * t118 * t77831;
    let t77834 = t15547 * t2085;
    let t77835 = 0.90915538847484472429e-2 * t77834;
    let t77836 = 0.44903406381989282115e-1 * t76143;
    let t77837 = 0.2993560425465952141e-1 * t76145;
    let t77838 = t14512 * t8568;
    let t77839 = 0.68186654135613354322e-2 * t77838;
    let t77840 = t71876 * t2344;
    let t77841 = 0.10227998120342003148e-1 * t77840;
    let t77842 = t14516 * t8526;
    let t77843 = 0.10227998120342003148e-1 * t77842;
    let t77844 = t71882 * t2329;
    (t77830, t77831, t77833, t77835, t77836, t77837, t77839, t77841, t77843, t77844)
}
