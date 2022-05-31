
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo16845(_: S2, _: S4, _: S7) {}
        
        fn test16845() { foo16845(S5, S3, S2, S6, S7, S8); }
    