
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo16494(_: S1, _: S2, _: S3, _: S4, _: S5, _: S6, _: S7, _: S8) {}
        
        fn test16494() { foo16494(S8, S1, S7, S2); }
    