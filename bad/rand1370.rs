
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo1370(_: S1, _: S4, _: S3, _: S7, _: S2, _: S6) {}
        
        fn test1370() { foo1370(S1, S0, S7, S5, S2, S3, S7); }
    