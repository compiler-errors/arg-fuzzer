
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo4712(_: S7, _: S1) {}
        
        fn test4712() { foo4712(S4, S2, S3, S5, S2, S7, S6); }
    