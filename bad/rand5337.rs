
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo5337(_: S2, _: S4, _: S2) {}
        
        fn test5337() { foo5337(S1, S8, S6, S2, S3); }
    