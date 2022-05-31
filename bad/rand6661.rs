
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo6661(_: S2, _: S3, _: S7, _: S5) {}
        
        fn test6661() { foo6661(S6, S2, S4, S3, S1, S6, S0); }
    