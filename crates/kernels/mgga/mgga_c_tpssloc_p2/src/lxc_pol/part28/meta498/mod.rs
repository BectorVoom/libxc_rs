//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta498 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1720;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1721;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1722;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta498<F: Float>(t26656: F, t4182: F, t7823: F, t814: F, t829: F, t25293: F, t25317: F, t226: F, t23187: F, t25274: F, t25285: F, t25289: F, t25301: F, t25304: F, t25308: F, t25310: F, t25314: F, t25322: F, t25326: F, t26613: F, t26654: F, t4281: F, t4291: F, t7839: F, t808: F, t812: F, t26611: F, t858: F, t25036: F, t25042: F, t25047: F, t25056: F, t25061: F, t2597: F, t26582: F, t26591: F, t2713: F, t4147: F, t4268: F, t4273: F, t7087: F, t7092: F, t7107: F, t7830: F, t855: F, t2053: F, t2718: F, t4300: F, t13463: F, t1528: F, t2054: F, t23207: F, t23209: F, t23233: F, t23236: F, t24291: F, t24305: F, t25194: F, t4301: F, t7842: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t26657, t26661, t26662, t26676, t26678) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1720::<F>(t26656, t4182, t7823, t814, t829, t25293, t25317, t226, t23187, t25274, t25285, t25289, t25301, t25304, t25308, t25310, t25314, t25322, t25326, t26613, t26654, t4281, t4291, t7839, t808, t812);
        let (t26679, t26680, t26684) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1721::<F>(t26611, t26678, t858, t25036, t25042, t25047, t25056, t25061, t2597, t26582, t26591, t2713, t4147, t4268, t4273, t7087, t7092, t7107, t7830, t855);
        let (t26690, t26698) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1722::<F>(t2053, t2718, t4300, t13463, t1528, t2054, t23207, t23209, t23233, t23236, t24291, t24305, t25194, t2713, t4147, t4268, t4301, t7087, t7092, t7107, t7842, t855);
    (t26657, t26661, t26662, t26676, t26679, t26680, t26684, t26690, t26698)
}
