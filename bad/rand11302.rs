
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo11302(_: S2, _: S2, _: S1, _: S6) {}
        
        fn test11302() { foo11302(S0, S3, S0, S2, S3, S5, S2); }
    