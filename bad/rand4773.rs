
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo4773(_: S5, _: S6, _: S2) {}
        
        fn test4773() { foo4773(S7, S5, S0, S1); }
    