//! MGGA_X_GDME vxc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_gdme.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn mgga_x_gdme_vxc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    vlapl: &mut Array<f64>,
    vtau: &mut Array<f64>,
    param_AA: f64,
    param_BB: f64,
    param_a: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let lapl0 = lapl[ip * 2];
        let lapl1 = lapl[ip * 2 + 1];
        let tau0 = tau[ip * 2];
        let tau1 = tau[ip * 2 + 1];
        let t2 = rho0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t5 = 1.0 / t4;
        let t6 = t3 * t5;
        let t7 = rho0 + rho1;
        let t8 = 1.0 / t7;
        let t11 = 2.0 * rho0 * t8 <= zeta_threshold;
        let t12 = zeta_threshold - 1.0;
        let t15 = 2.0 * rho1 * t8 <= zeta_threshold;
        let t16 = -t12;
        let t17 = rho0 - rho1;
        let t19 = piecewise5(t11, t12, t15, t16, t17 * t8);
        let t20 = 1.0 + t19;
        let t21 = t20 <= zeta_threshold;
        let t22 = pow_1_3(zeta_threshold);
        let t23 = t22 * zeta_threshold;
        let t24 = pow_1_3(t20);
        let t26 = piecewise3(t21, t23, t24 * t20);
        let t27 = pow_1_3(t7);
        let t28 = t26 * t27;
        let t31 = M_CBRT2;
        let t34 = pow_1_3(1.0 / M_PI);
        let t35 = 1.0 / t34;
        let t36 = M_CBRT4;
        let t37 = t35 * t36;
        let t38 = M_PI * M_PI;
        let t39 = pow_1_3(t38);
        let t40 = t39 * t39;
        let t44 = 2.0 / 9.0 * (param_AA + 3.0 / 5.0 * param_BB) * t31 * t37 / t40;
        let t46 = param_BB * t3 * t35;
        let t47 = t31 * t31;
        let t48 = t36 * t47;
        let t50 = 1.0 / t39 / t38;
        let t51 = param_a * param_a;
        let t52 = t51 - param_a + 1.0 / 2.0;
        let t53 = t52 * lapl0;
        let t54 = pow_1_3(rho0);
        let t55 = t54 * t54;
        let t57 = 1.0 / t55 / rho0;
        let t66 = t44 + t46 * t48 * t50 * (t53 * t57 - 2.0 * t57 * tau0) / 27.0;
        let t70 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t28 * t66);
        let t71 = rho1 <= dens_threshold;
        let t72 = -t17;
        let t74 = piecewise5(t15, t12, t11, t16, t72 * t8);
        let t75 = 1.0 + t74;
        let t76 = t75 <= zeta_threshold;
        let t77 = pow_1_3(t75);
        let t79 = piecewise3(t76, t23, t77 * t75);
        let t80 = t79 * t27;
        let t81 = t52 * lapl1;
        let t82 = pow_1_3(rho1);
        let t83 = t82 * t82;
        let t85 = 1.0 / t83 / rho1;
        let t94 = t44 + t46 * t48 * t50 * (t81 * t85 - 2.0 * t85 * tau1) / 27.0;
        let t98 = piecewise3(t71, 0.0, -3.0 / 8.0 * t6 * t80 * t94);
        let tzk0 = t70 + t98;
        zk[ip] += tzk0;
        let t99 = t7 * t7;
        let t100 = 1.0 / t99;
        let t101 = t17 * t100;
        let t103 = piecewise5(t11, 0.0, t15, 0.0, t8 - t101);
        let t106 = piecewise3(t21, 0.0, 4.0 / 3.0 * t24 * t103);
        let t107 = t106 * t27;
        let t111 = t27 * t27;
        let t112 = 1.0 / t111;
        let t113 = t26 * t112;
        let t116 = t6 * t113 * t66 / 8.0;
        let t117 = t3 * t3;
        let t118 = t117 * t5;
        let t120 = t118 * t28 * param_BB;
        let t121 = t47 * t50;
        let t122 = rho0 * rho0;
        let t124 = 1.0 / t55 / t122;
        let t131 = t37 * t121 * (-5.0 / 3.0 * t53 * t124 + 10.0 / 3.0 * tau0 * t124);
        let t135 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t107 * t66 - t116 - t120 * t131 / 72.0);
        let t136 = t72 * t100;
        let t138 = piecewise5(t15, 0.0, t11, 0.0, -t8 - t136);
        let t141 = piecewise3(t76, 0.0, 4.0 / 3.0 * t77 * t138);
        let t142 = t141 * t27;
        let t146 = t79 * t112;
        let t149 = t6 * t146 * t94 / 8.0;
        let t151 = piecewise3(t71, 0.0, -3.0 / 8.0 * t6 * t142 * t94 - t149);
        let tvrho0 = t70 + t98 + t7 * (t135 + t151);
        vrho[ip * 2] += tvrho0;
        let t155 = piecewise5(t11, 0.0, t15, 0.0, -t8 - t101);
        let t158 = piecewise3(t21, 0.0, 4.0 / 3.0 * t24 * t155);
        let t159 = t158 * t27;
        let t164 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t159 * t66 - t116);
        let t166 = piecewise5(t15, 0.0, t11, 0.0, t8 - t136);
        let t169 = piecewise3(t76, 0.0, 4.0 / 3.0 * t77 * t166);
        let t170 = t169 * t27;
        let t175 = t118 * t80 * param_BB;
        let t176 = rho1 * rho1;
        let t178 = 1.0 / t83 / t176;
        let t185 = t37 * t121 * (-5.0 / 3.0 * t81 * t178 + 10.0 / 3.0 * tau1 * t178);
        let t189 = piecewise3(t71, 0.0, -3.0 / 8.0 * t6 * t170 * t94 - t149 - t175 * t185 / 72.0);
        let tvrho1 = t70 + t98 + t7 * (t164 + t189);
        vrho[ip * 2 + 1] += tvrho1;
        let tvsigma0 = 0.0;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let tvsigma2 = 0.0;
        vsigma[ip * 3 + 2] += tvsigma2;
        let t192 = t37 * t47;
        let t193 = t50 * t52;
        let t195 = t192 * t193 * t57;
        let t198 = piecewise3(t2, 0.0, -t120 * t195 / 72.0);
        let tvlapl0 = t7 * t198;
        vlapl[ip * 2] += tvlapl0;
        let t200 = t192 * t193 * t85;
        let t203 = piecewise3(t71, 0.0, -t175 * t200 / 72.0);
        let tvlapl1 = t7 * t203;
        vlapl[ip * 2 + 1] += tvlapl1;
        let t205 = t37 * t121 * t57;
        let t208 = piecewise3(t2, 0.0, t120 * t205 / 36.0);
        let tvtau0 = t7 * t208;
        vtau[ip * 2] += tvtau0;
        let t210 = t37 * t121 * t85;
        let t213 = piecewise3(t71, 0.0, t175 * t210 / 36.0);
        let tvtau1 = t7 * t213;
        vtau[ip * 2 + 1] += tvtau1;
    }
}
