
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo8856(_: S8, _: S6, _: S2, _: S4) {}
        
        fn test8856() { foo8856(S1, S3, S4, S6, S7, S8); }
    