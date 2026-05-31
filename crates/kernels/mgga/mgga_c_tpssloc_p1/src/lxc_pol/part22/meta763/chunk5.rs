//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2575/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2575<F: Float>(t1129: F, t1137: F, t15121: F, t15141: F, t1695: F, t18644: F, t18840: F, t18894: F, t18899: F, t21855: F, t21887: F, t21890: F, t3327: F, t436: F, t44172: F, t44214: F, t4797: F, t4820: F, t4858: F, t51392: F, t51599: F, t6053: F, t6056: F, t6085: F, t63597: F, t71876: F, t71879: F, t71902: F, t71915: F, t71929: F, t71941: F, t71955: F, t71968: F, t71978: F, t71989: F, t72019: F, t72037: F) -> F {
    let t72041 = F::cast_from(0.17544670867903938621e1_f64) * t63597 * t1695 + F::cast_from(0.17544670867903938621e1_f64) * t18899 * t4858 + F::cast_from(0.17544670867903938621e1_f64) * t15121 * t6085 + t71876 - t71879 + F::cast_from(3.0_f64) * t18840 * t4820 + F::cast_from(3.0_f64) * t15141 * t6053 + F::cast_from(3.0_f64) * t4797 * t18894 + F::cast_from(0.96491876992155210402e2_f64) * t51599 * t6056 - F::cast_from(0.19298375398431042081e3_f64) * t44214 * t21855 + F::cast_from(1.0_f64) * t3327 * t21887 + F::cast_from(1.0_f64) * t1129 * (t71902 + t71915 + t71929 + t71941 + t71955 + t71968 + t71978 + t71989) * t1137 + F::cast_from(0.2069040516770936012e4_f64) * t44172 * t21890 - F::cast_from(0.57895126195293126241e3_f64) * t51392 * t18644 - F::cast_from(0.310907e-1_f64) * (t72019 + t72037) * t436;
    t72041
}
