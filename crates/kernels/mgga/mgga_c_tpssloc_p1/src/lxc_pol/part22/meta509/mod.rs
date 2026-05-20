//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta509 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1960;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1961;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1962;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta509<F: Float>(t1683: F, t6052: F, t1682: F, t18643: F, t6036: F, t3359: F, t11314: F, t11317: F, t14702: F, t14766: F, t18203: F, t18219: F, t18229: F, t18494: F, t18505: F, t18512: F, t21739: F, t21741: F, t21747: F, t21751: F, t21760: F, t21764: F, t21767: F, t21771: F, t21774: F, t21778: F, t21781: F, t21783: F, t21786: F, t21789: F, t21792: F, t21795: F, t21802: F, t21804: F, t1137: F, t11352: F, t1671: F, t6020: F, t3264: F, t1129: F, t11350: F, t11420: F, t15146: F, t1695: F, t18840: F, t18899: F, t21726: F, t21728: F, t21812: F, t21815: F, t21836: F, t21839: F, t3332: F, t3357: F, t3376: F, t3401: F, t4797: F, t6053: F, t6056: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t21842, t21845, t21854, t21855, t21870) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1960::<F>(t1683, t6052, t1682, t18643, t6036, t3359, t11314, t11317, t14702, t14766, t18203, t18219, t18229, t18494, t18505, t18512, t21739, t21741, t21747, t21751);
        let t21885 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1961::<F>(t21760, t21764, t21767, t21771, t21774, t21778, t21781, t21783, t21786, t21789, t21792, t21795, t21802, t21804);
        let (t21886, t21887, t21890, t21895, t21897, t21898) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1962::<F>(t21870, t21885, t1137, t11352, t21854, t1671, t6020, t3264, t1129, t11350, t11420, t15146, t1683, t1695, t18840, t18899, t21726, t21728, t21812, t21815, t21836, t21839, t21842, t21845, t21855, t3332, t3357, t3376, t3401, t4797, t6053, t6056);
    (t21842, t21845, t21854, t21855, t21886, t21887, t21890, t21895, t21897, t21898)
}
