
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo16071(_: S4, _: S6, _: S8) {}
        
        fn test16071() { foo16071(S1, S2, S4, S7, S8); }
    