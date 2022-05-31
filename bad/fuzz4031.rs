
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo4031(_: S6, _: S4, _: S2) {}
        
        fn test4031() { foo4031(S1, S3, S1, S6, S8, S7); }
    