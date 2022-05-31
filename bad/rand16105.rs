
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo16105(_: S1, _: S4, _: S8) {}
        
        fn test16105() { foo16105(S7, S6, S4, S1, S4, S2, S2); }
    