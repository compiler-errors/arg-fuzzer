
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo16176(_: S0, _: S4, _: S5, _: S0, _: S3) {}
        
        fn test16176() { foo16176(S3, S6, S8, S2, S4, S7, S5, S1); }
    