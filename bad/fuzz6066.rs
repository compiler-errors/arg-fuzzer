
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo6066(_: S3, _: S5, _: S6, _: S8) {}
        
        fn test6066() { foo6066(S1, S2, S5, S6, S7); }
    